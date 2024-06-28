use tokio::{fs::OpenOptions, io::AsyncWriteExt};

pub enum Actions {
    Transaction {
        from: String,
        to: String,
        amount: String,
    },
    Verification {
        user: String,
    },
    BalanceCheck {
        user: String,
    },
    GetLogs {
        user: String,
    },
}

use std::io::Write;

pub async fn log(action: Actions, was_successful: bool) {
    let time = curr_time();
    match action {
        Actions::Transaction { from, to, amount } => {
            //create file if it does not exist
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open("requests.log")
                .await
                .unwrap();
            let mut buf: Vec<u8> = Vec::<u8>::new();
            writeln!(
                buf,
                "{}: Transaction from {} to {} for amount {} : Action was {}",
                time,
                from,
                to,
                amount,
                if was_successful {
                    "successful"
                } else {
                    "unsuccessful"
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
                "{}: Verification for user {} : Action was {}",
                time,
                user,
                if was_successful {
                    "successful"
                } else {
                    "unsuccessful"
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
                "{}: Balance check for user {}: Action was {}",
                time,
                user,
                if was_successful {
                    "successful"
                } else {
                    "unsuccessful"
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
                "{}: Get logs for user {} : Action was {}",
                time,
                user,
                if was_successful {
                    "successful"
                } else {
                    "unsuccessful"
                }
            )
            .unwrap();
            file.write_all(&buf).await.unwrap();
        }
    }
}
pub fn curr_time() -> String {
    let now = chrono::Local::now();
    date.to_string()
}
