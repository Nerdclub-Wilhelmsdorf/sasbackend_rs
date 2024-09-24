use crate::errors::BackendError;
use crate::DB;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use serde::Deserialize;
use surrealdb::opt::PatchOp;
use surrealdb::sql::Thing;
#[derive(Deserialize)]
pub struct AccountID {
    pub id: Thing,
}

#[derive(Deserialize)]
pub struct DBUser {
    pub id: AccountID,
    pub name: String,
    pub balance: String,
    pub pin: String,
    pub transactions: String,
}

impl DBUser {
    pub async fn fetch_user(id: &String) -> Result<Option<DBUser>, surrealdb::Error> {
        let user: Option<DBUser> = DB.select(("user", id)).await?;
        Ok(user)
    }
    pub async fn has_sufficient_funds(&self, amount: &str) -> bool {
        let balance = Decimal::from_str(&self.balance);
        let balance = match balance {
            Ok(balance) => balance,
            Err(_) => return false,
        };
        let amount = Decimal::from_str(amount);
        let amount = match amount {
            Ok(amount) => amount,
            Err(_) => return false,
        };
        balance >= amount
    }
    pub async fn update_value(
        &self,
        key: &str,
        value: &str,
    ) -> Result<Option<DBUser>, BackendError> {
        let id = self.id.id.clone(); // Clone the id value
        let updated_user: Option<DBUser> = DB
            .update(("user", id.to_string())) // Use the cloned id value
            .patch(PatchOp::replace(&format!("/{}", key), value))
            .await?;

        Ok(updated_user)
    }
    pub async fn update_balance(
        &self,
        amount: &str,
        transfer_type: TransferType,
    ) -> Result<Option<DBUser>, BackendError> {
        let id = self.id.id.clone(); // Clone the id value
        let current_user_state: Option<DBUser> = DB.select(("user", id.to_string().clone())).await?;
        let current_user_state = match current_user_state {
            Some(current_user_state) => current_user_state,
            None => return Ok(None),
        };
        let current_balance = Decimal::from_str(&current_user_state.balance);
        let current_balance = match current_balance {
            Ok(current_balance) => current_balance,
            Err(_) => return Ok(None),
        };
        let amount = Decimal::from_str(amount);
        let amount = match amount {
            Ok(amount) => amount,
            Err(_) => return Ok(None),
        };
        let new_balance: Decimal = match transfer_type {
            TransferType::Add => current_balance + amount,
            TransferType::Subtract => current_balance - amount,
        };
        if new_balance < dec!(0) {
            return Ok(None);
        }
        let new_balance = new_balance.to_string();
        let updated_user: Option<DBUser> = DB
            .update(("user", id.to_string()))
            .patch(PatchOp::replace(&format!("/{}", "balance"), new_balance))
            .await?;
        Ok(updated_user)
    }
}

pub enum TransferType {
    Add,
    Subtract,
}

pub fn verify_pin(database_pin: &str, input_pin: &str) -> Result<bool, BackendError> {
    let verified = bcrypt::verify(input_pin, database_pin);
    match verified {
        Ok(verified) => Ok(verified),
        Err(e) => Err(BackendError::BcryptError(e)),
    }
}
