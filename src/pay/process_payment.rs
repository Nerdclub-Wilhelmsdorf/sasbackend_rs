use std::str::FromStr;

use super::{log_transaction, payment_errors::PaymentError};
use crate::{
    user::{verify_pin, DBUser, TransferType},
    TAX_FACTOR,
};
use rust_decimal::Decimal;
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
    payload: PaymentRequest,
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
    if bank.is_none() {
        return Ok(Err(PaymentError::UserNotFound("zentralbank".to_string())));
    }
    let sender = sender.unwrap();
    let receiver = receiver.unwrap();
    let bank = bank.unwrap();
    if sender.id.id == receiver.id.id {
        return Ok(Err(PaymentError::SameUser));
    }
    if !verify_pin(&sender.pin, &payload.pin) {
        return Ok(Err(PaymentError::IncorrectPin));
    }
    let tax = Decimal::from_str(TAX_FACTOR).unwrap();
    let amount = Decimal::from_str(&payload.amount).unwrap();
    let tax_amount = amount * tax;
    let tax_amount_bank: Decimal = tax_amount.clone() - amount;
    let tax_amount = tax_amount.to_string();
    let tax_amount_bank = tax_amount_bank.to_string();
    if !sender.has_sufficient_funds(&tax_amount).await {
        return Ok(Err(PaymentError::InsufficientFunds));
    }
    match sender
        .update_balance(&tax_amount, TransferType::Subtract)
        .await
    {
        Ok(_) => {}
        Err(_) => return Ok(Err(PaymentError::FailedMoneyTransfer)),
    }
    match receiver
        .update_balance(&payload.amount, TransferType::Add)
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
    match log_transaction::log_transaction(&payload, sender, receiver, bank).await {
        Ok(_) => {}
        Err(_) => return Ok(Err(PaymentError::FailedMoneyTransfer)),
    }
    return Ok(Ok("success".to_string()));
}
