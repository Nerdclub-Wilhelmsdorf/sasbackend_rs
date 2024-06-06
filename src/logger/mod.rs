use tokio::fs::OpenOptions;

pub enum Actions{
    Transaction{from: String, to: String, amount: f64},
    Verification{user: String},
    BalanceCheck{user: String}, 
    GetLogs{user: String}
}




fn write_to_file(action: Actions) {
    match action {
        Actions::Transaction{from, to, amount} => {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("transactions.log")
                .unwrap();
            writeln!(file, "Transaction from {} to {} with amount {}", from, to, amount).unwrap();
        },
        Actions::Verification(user) => {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("transactions.log")
                .unwrap();
            writeln!(file, "Verification for user {}", user).unwrap();
        },
        Actions::BalanceCheck(user) => {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("transactions.log")
                .unwrap();
            writeln!(file, "Balance check for user {}", user).unwrap();
        },
        Actions::GetLogs(user) => {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("transactions.log")
                .unwrap();
            writeln!(file, "Get logs for user {}", user).unwrap();
        }
    }
}