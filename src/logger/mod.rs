use tokio::{fs::OpenOptions, io::AsyncWriteExt};

pub enum Actions{
    Transaction{from: String, to: String, amount: f64},
    Verification{user: String},
    BalanceCheck{user: String}, 
    GetLogs{user: String}
}

use std::io::Write;


pub async fn log(action: Actions) {
    match action {
        Actions::Transaction{from, to, amount} => {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("requests.log")
                .await.unwrap();
            let mut buf: Vec<u8> = Vec::<u8>::new();
            writeln!(buf, "Transaction from {} to {} for amount {}", from, to, amount).unwrap();
            file.write_all(&buf).await.unwrap();
        },
        Actions::Verification{user} => {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("requests.log")
                .await.unwrap();
            let mut buf: Vec<u8> = Vec::<u8>::new();
            writeln!(buf, "Verification for user {}", user).unwrap();
            file.write_all(&buf).await.unwrap();

        },
        Actions::BalanceCheck{user }=> {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("requests.log")
                .await.unwrap();
            let mut buf: Vec<u8> = Vec::<u8>::new();
            writeln!(buf, "Balance check for user {}", user).unwrap();
            file.write_all(&buf).await.unwrap();

        },
        Actions::GetLogs{user} => {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("requests.log")
                .await.unwrap();
            let mut buf: Vec<u8> = Vec::<u8>::new();
            writeln!(buf, "Get logs for user {}", user).unwrap();
            file.write_all(&buf).await.unwrap();

        }
    }
}