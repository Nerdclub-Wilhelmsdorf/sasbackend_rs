use std::str::FromStr;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Serialize;

use crate::{logger, user::DBUser, TAX_FACTOR};

use super::process_payment::PaymentRequest;

#[derive(Serialize)]
struct TransactionLog<'a> {
    time: &'a String,
    from: &'a String,
    to: &'a String,
    amount: &'a String,
}

pub async fn log_transaction(
    payload: &PaymentRequest,
    sender: DBUser,
    receiver: DBUser,
    bank: DBUser,
) -> Result<(), surrealdb::Error> {
    let time = logger::curr_time();
    let tax: Decimal = Decimal::from_str(TAX_FACTOR).unwrap();
    let amount = Decimal::from_str(&payload.amount).unwrap();
    let tax_amount = amount - (amount * (dec!(1) / Decimal::from_str(TAX_FACTOR).unwrap()));
    let tax_amount_bank: Decimal = amount / tax;
    let tax_amount = tax_amount.to_string();
    let amount = amount.to_string();
    let tax_amount_bank = tax_amount_bank.to_string();

    let transaction_reciever = TransactionLog {
        time: &time,
        from: &sender.id.id.to_string(),
        to: &receiver.id.id.to_string(),
        amount: &tax_amount,
    };
    let transaction_reciever = serde_json::to_string(&transaction_reciever).unwrap();
    let transaction_sender = TransactionLog {
        time: &time,
        from: &sender.id.id.to_string(),
        to: &receiver.id.id.to_string(),
        amount: &payload.amount,
    };
    let transaction_sender = serde_json::to_string(&transaction_sender).unwrap();
    let transaction_bank = TransactionLog {
        time: &time,
        from: &sender.id.id.to_string(),
        to: &receiver.id.id.to_string(),
        amount: &(Decimal::from_str(&payload.amount).unwrap()
            * Decimal::from_str(TAX_FACTOR).unwrap()
            - Decimal::from_str(&payload.amount).unwrap())
        .to_string(),
    };
    let transaction_bank = serde_json::to_string(&transaction_bank).unwrap();
    let mut sender_transactions: Vec<String> = sender
        .transactions
        .split("###")
        .map(|s| s.to_string())
        .collect();
    let mut receiver_transactions: Vec<String> = receiver
        .transactions
        .split("###")
        .map(|s| s.to_string())
        .collect();
    let mut bank_transactions: Vec<String> = bank
        .transactions
        .split("###")
        .map(|s| s.to_string())
        .collect();
    sender_transactions.push(transaction_sender);
    receiver_transactions.push(transaction_reciever);
    bank_transactions.push(transaction_bank);
    let sender_transactions: String = sender_transactions.join("###");
    let receiver_transactions: String = receiver_transactions.join("###");
    let bank_transactions: String = bank_transactions.join("###");
    sender
        .update_value("transactions", &sender_transactions)
        .await?;
    receiver
        .update_value("transactions", &receiver_transactions)
        .await?;
    bank.update_value("transactions", &bank_transactions)
        .await?;
    Ok(())
}
