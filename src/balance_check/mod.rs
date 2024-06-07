mod balance_request;

use salvo::prelude::*;

use crate::{
    logger,
    user::{verify_pin, DBUser},
};

#[handler]
pub async fn balance_check(req: &mut Request, res: &mut Response) {
    let payload_result = req.parse_json::<balance_request::BalanceCheck>().await;
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
    let user = DBUser::fetch_user(&payload.acc1).await;
    match user {
        Ok(user) => match user {
            Some(user) => {
                if !verify_pin(&user.pin, &payload.pin) {
                    res.status_code(StatusCode::CREATED);
                    logger::log(
                        logger::Actions::GetLogs {
                            user: &payload.acc1,
                        },
                        logger::Return::Failed,
                    )
                    .await;
                    return res.render("wrong pin");
                }
                res.status_code(StatusCode::OK);
                res.render(user.balance);
                logger::log(
                    logger::Actions::GetLogs {
                        user: &payload.acc1,
                    },
                    logger::Return::Success,
                )
                .await;
                return;
            }
            None => {
                res.status_code(StatusCode::CREATED);
                res.render("user not found (no row)");
            }
        },
        Err(_) => {
            res.status_code(StatusCode::CREATED);
            res.render("Failed to connect to the database");
        }
    }
    logger::log(
        logger::Actions::BalanceCheck {
            user: &payload.acc1,
        },
        logger::Return::Failed,
    )
    .await;
}
