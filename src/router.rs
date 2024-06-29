use salvo::Router;

use crate::{balance_check, get_logs, hello, pay, verify_account};

pub fn get_router() -> Router
{
let root_route = Router::new().get(hello);
let pay_route = Router::new().path("/pay").post(pay::pay);
let balance_route = Router::new()
    .path("/balanceCheck")
    .post(balance_check::balance_check);
let log_route = Router::new().path("/getLogs").post(get_logs::get_logs);
let verify_route = Router::new()
    .path("/verify")
    .post(verify_account::verify_account);
Router::new()
    .push(root_route)
    .push(pay_route)
    .push(log_route)
    .push(verify_route)
    .push(balance_route)

}