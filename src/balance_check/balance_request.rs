use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BalanceCheck {
    pub acc1: String,
    pub pin: String,
}

impl BalanceCheck {
    pub fn validate(&self) -> Option<String> {
        if self.acc1.is_empty() || self.pin.is_empty() {
            return Some("empty fields".to_string());
        }
        if self.pin.parse::<i32>().is_err() || self.pin.len() != 4 {
            return Some("invalid pin".to_string());
        }
        None
    }
}
