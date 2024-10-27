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
        let content = match self {
            BackendError::DataBaseError(e) => format!("Backend error: {:?}", e.to_string()),
            BackendError::PaymentError(e) => format!("Backend error: {:?}", e.to_string()),
            BackendError::BcryptError(e) => format!("Backend error: {:?}", e.to_string()),
        };
        write!(f, "{}", content)
    }
}



#[derive(Debug)]
pub enum PaymentError {
    //add lifetime to the user id
    UserNotFound(String),
    IncorrectPin,
    InsufficientFunds,
    RecieverIsGuest,
    FailedMoneyTransfer,
    SameUser,
    ReceiverIsGuest
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
            PaymentError::RecieverIsGuest => write!(f, "Reciever is a guest account"),
            PaymentError::SameUser => write!(f, "Sender and receiver are the same"),
            PaymentError::ReceiverIsGuest => write!(f, "Receiver is a guest account"),
        }
    }
}



impl From<surrealdb::Error> for BackendError {
    fn from(e: surrealdb::Error) -> Self {
        BackendError::DataBaseError(e)
    }
}
