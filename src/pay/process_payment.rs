use std::str::FromStr;

use super::{log_transaction, payment_errors::PaymentError};
use crate::{
    lock_user::{self, unlock},
    user::{verify_pin, DBUser, TransferType},
    TAX_FACTOR,
};
use lock_user::increment_failed_attempts;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use salvo::Request;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct PaymentRequest {
    pub from: String,
    pub to: String,
    pub amount: String,
    pub pin: String,
}

impl PaymentRequest {
    pub fn validate(&self) -> Option<String> {
        if self.from.is_empty()
            || self.to.is_empty()
            || self.amount.is_empty()
            || self.pin.is_empty()
        {
            return Some("empty fields".to_string());
        }
        if self.amount.parse::<f64>().is_err() {
            return Some("invalid amount".to_string());
        }
        if self.amount.parse::<f64>().unwrap() <= 0.0 {
            return Some("amount must be greater than 0".to_string());
        }
        if self.from == self.to {
            return Some("sender and receiver are the same".to_string());
        }
        if self.pin.parse::<i32>().is_err() || self.pin.len() != 4 {
            return Some("invalid pin".to_string());
        }
        None
    }
}

pub async fn process_payment(
    payload: &PaymentRequest,
    Request: &mut Request,
) -> Result<Result<String, PaymentError>, surrealdb::Error> {
    let sender = DBUser::fetch_user(&payload.from).await?;
    let receiver = DBUser::fetch_user(&payload.to).await?;
    let bank = DBUser::fetch_user(&"zentralbank".to_string()).await?;
    if sender.is_none() {
        return Ok(Err(PaymentError::UserNotFound(payload.from.clone())));
    }
    if receiver.is_none() {
        return Ok(Err(PaymentError::UserNotFound(payload.to.clone())));
    }
    let zentralbank = "zentralbank".to_string();
    if bank.is_none() {
        return Ok(Err(PaymentError::UserNotFound(zentralbank)));
    }
    let sender = sender.unwrap();
    let receiver = receiver.unwrap();
    let bank = bank.unwrap();
    if sender.id.id == receiver.id.id {
        return Ok(Err(PaymentError::SameUser));
    }
    if !verify_pin(&sender.pin, &payload.pin) {
        increment_failed_attempts(Request.remote_addr().to_owned()).await;
        return Ok(Err(PaymentError::IncorrectPin));
    }
    unlock(Request.remote_addr().to_owned()).await;
    let tax: Decimal = Decimal::from_str(TAX_FACTOR).unwrap();
    let amount = Decimal::from_str(&payload.amount).unwrap();
    let tax_amount = amount - (amount * (dec!(1) / Decimal::from_str(TAX_FACTOR).unwrap()));
    let tax_amount_bank: Decimal = amount / tax;
    let tax_amount = tax_amount.to_string();
    let amount = amount.to_string();
    let tax_amount_bank = tax_amount_bank.to_string();
    if !sender.has_sufficient_funds(&amount).await {
        return Ok(Err(PaymentError::InsufficientFunds));
    }
    match sender.update_balance(&amount, TransferType::Subtract).await {
        Ok(_) => {}
        Err(_) => return Ok(Err(PaymentError::FailedMoneyTransfer)),
    }
    match receiver
        .update_balance(&tax_amount, TransferType::Add)
        .await
    {
        Ok(_) => {}
        Err(_) => return Ok(Err(PaymentError::InsufficientFunds)),
    }
    match bank
        .update_balance(&tax_amount_bank, TransferType::Add)
        .await
    {
        Ok(_) => {}
        Err(_) => return Ok(Err(PaymentError::FailedMoneyTransfer)),
    }
    match log_transaction::log_transaction(payload, sender, receiver, bank).await {
        Ok(_) => {}
        Err(_) => return Ok(Err(PaymentError::FailedMoneyTransfer)),
    }
    Ok(Ok("success".to_string()))
}
