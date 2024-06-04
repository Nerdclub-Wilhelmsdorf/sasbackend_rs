use std::str::FromStr;

use rust_decimal::Decimal;
use salvo::prelude::*;
use serde::Deserialize;
mod payment_errors;
use self::payment_errors::PaymentError;
use crate::user::{DBUser, TransferType};

//pay function: TODO
//desirialize the request   DONE
//fetch the sender and the receiver DONE
//check if the sender is locked WIP
//check if the senders password is correct DONE
//check if the sender has enough money DONE
//subtract the sum from the sender (multiply by tax factor) //DONE
//add the sum to the receiver (multiply by tax factor)  //DONE
//add the tax to the bank //DONE
//remove the "failed attempts" status from the sender //WIP
//log the transaction for all the parties //WIP
//return "success" //DONE
//if any of the checks fail, return "failed" //DONE

#[handler]
pub async fn pay(req: &mut Request, res: &mut Response) {
    let payload_result = req.parse_json::<PaymentRequest>().await;
    let payload = match payload_result {
        Ok(payload) => payload,
        Err(_) => {
            res.status_code(StatusCode::CREATED);
            return res.render("Failed to parse the request, are the values set according to the API documentation?");
        }
    };
    let payment = process_payment(payload).await;
    match payment {
        Ok(payment) => match payment {
            Ok(_) => {
                res.status_code(StatusCode::OK);
                res.render("Success");
            }
            Err(e) => {
                res.status_code(StatusCode::CREATED);
                res.render(e.to_string());
            }
        },
        Err(_) => {
            res.status_code(StatusCode::CREATED);
            res.render("Failed to connect to the database");
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PaymentRequest {
    pub from: String,
    pub to: String,
    pub amount: String,
    pub pin: String,
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
    if sender.id.id == receiver.id.id {
        return Ok(Err(PaymentError::SameUser));
    }
    if !verify_pin(&sender.pin, &payload.pin) {
        return Ok(Err(PaymentError::IncorrectPin));
    }
    let tax = Decimal::from_str("1.1").unwrap();
    let amount = Decimal::from_str(&payload.amount).unwrap();
    let tax_amount = amount * tax;
    let tax_amount_bank: Decimal = tax_amount.clone() - amount;
    let tax_amount = tax_amount.to_string();
    let tax_amount_bank = tax_amount_bank.to_string();
    if !sender.has_sufficient_funds(&tax_amount).await {
        return Ok(Err(PaymentError::InsufficientFunds))
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
        .unwrap()
        .update_balance(&tax_amount_bank, TransferType::Add)
        .await
    {
        Ok(_) => {}
        Err(_) => return Ok(Err(PaymentError::FailedMoneyTransfer)),
    }
    return Ok(Ok("success".to_string()));
}

fn verify_pin(database_pin: &str, input_pin: &str) -> bool {
    bcrypt::verify(input_pin, database_pin).unwrap()
    //TODO
}
