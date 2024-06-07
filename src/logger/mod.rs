use tokio::{fs::OpenOptions, io::AsyncWriteExt};

pub enum Actions<'a> {
    Transaction {
        from: &'a str,
        to: &'a str,
        amount: &'a str,
    },
    Verification {
        user: &'a str,
    },
    BalanceCheck {
        user: &'a str,
    },
    GetLogs {
        user: &'a str,
    },
}
pub enum Return {
    Success,
    Failed,
}
use std::io::Write;

pub async fn log(action: Actions<'_>, return_value: Return) {
    match action {
        Actions::Transaction { from, to, amount } => {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("requests.log")
                .await
                .unwrap();
            let mut buf: Vec<u8> = Vec::<u8>::new();
            writeln!(
                buf,
                "Transaction from {} to {} for amount {}: Action {}",
                from,
                to,
                amount,
                match return_value {
                    Return::Success => "succeeded",
                    Return::Failed => "failed",
                }
            )
            .unwrap();
            file.write_all(&buf).await.unwrap();
        }
        Actions::Verification { user } => {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("requests.log")
                .await
                .unwrap();
            let mut buf: Vec<u8> = Vec::<u8>::new();
            writeln!(
                buf,
                "Verification for user {}: Action {}",
                user,
                match return_value {
                    Return::Success => "succeeded",
                    Return::Failed => "failed",
                }
            )
            .unwrap();
            file.write_all(&buf).await.unwrap();
        }
        Actions::BalanceCheck { user } => {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("requests.log")
                .await
                .unwrap();
            let mut buf: Vec<u8> = Vec::<u8>::new();
            writeln!(
                buf,
                "Balance check for user {} : Action {}",
                user,
                match return_value {
                    Return::Success => "succeeded",
                    Return::Failed => "failed",
                }
            )
            .unwrap();
            file.write_all(&buf).await.unwrap();
        }
        Actions::GetLogs { user } => {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("requests.log")
                .await
                .unwrap();
            let mut buf: Vec<u8> = Vec::<u8>::new();
            writeln!(
                buf,
                "Get logs for user {}: Action {}",
                user,
                match return_value {
                    Return::Success => "succeeded",
                    Return::Failed => "failed",
                }
            )
            .unwrap();
            file.write_all(&buf).await.unwrap();
        }
    }
}
