#![allow(dead_code)]

use std::fmt;

use bcrypt::BcryptError;

#[derive(Debug)]
pub enum BackendError {
    DataBaseError(surrealdb::Error),
    PaymentError(PaymentError),
    BcryptError(BcryptError)
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Backend error: {:?}", self)
    }
}



#[derive(Debug)]
pub enum PaymentError {
    //add lifetime to the user id
    UserNotFound(String),
    IncorrectPin,
    InsufficientFunds,
    FailedMoneyTransfer,
    SameUser,
}

impl std::fmt::Display for PaymentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PaymentError::UserNotFound(id) => write!(f, "User not found: {} (error no row)", id),
            PaymentError::IncorrectPin => write!(f, "wrong pin"),
            PaymentError::InsufficientFunds => write!(f, "insufficient funds"),
            PaymentError::FailedMoneyTransfer => {
                write!(f, "Failed to add/remove money from account")
            }
            PaymentError::SameUser => write!(f, "Sender and receiver are the same"),
        }
    }
}



impl From<surrealdb::Error> for BackendError {
    fn from(e: surrealdb::Error) -> Self {
        BackendError::DataBaseError(e)
    }
}
