mod log_request;

use bcrypt::verify;
use salvo::prelude::*;

use crate::{logger, user::DBUser};

#[handler]
pub async fn get_logs(req: &mut Request, res: &mut Response) {
    let payload_result = req.parse_json::<log_request::GetLogs>().await;
    let payload = match payload_result {
        Ok(payload) => payload,
        Err(_) => {
            res.status_code(StatusCode::CREATED);
            return res.render("Failed to parse the request, are the values set according to the API documentation?");
        }
    };
    let payload = match payload.validate() {
        Some(e) => {
            res.status_code(StatusCode::CREATED);
            return res.render(e);
        }
        None => payload,
    };
    let user = DBUser::fetch_user(&payload.acc).await;

    match user {
        Ok(user) => match user {
            Some(user) => {
                let verified = verify(&payload.pin, &user.pin);
                let verified = match verified {
                    Ok(verified) => verified,
                    Err(_) =>{
                        return res.render("wrong pin")
                    }
                };
                
                if !verified {
                    res.status_code(StatusCode::CREATED);
                    logger::log(logger::Actions::BalanceCheck { user: payload.acc }, false).await;
                    return res.render("wrong pin");
                }
                res.status_code(StatusCode::OK);
                res.render(user.transactions);
                logger::log(logger::Actions::GetLogs { user: payload.acc }, true).await;
                return;
            }
            None => {
                res.status_code(StatusCode::CREATED);
                res.render("User not found (no row)");
            }
        },
        Err(_) => {
            res.status_code(StatusCode::CREATED);
            res.render("Failed to connect to the database");
        }
    }
    logger::log(logger::Actions::GetLogs { user: payload.acc }, false).await;
}
