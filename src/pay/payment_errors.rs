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
