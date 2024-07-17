mod verify_request;

use bcrypt::verify;
use salvo::prelude::*;

use crate::{
    lock_user::{increment_failed_attempts, unlock},
    logger,
    user::DBUser,
};

#[handler]
pub async fn verify_account(req: &mut Request, res: &mut Response) {
    let payload_result = req.parse_json::<verify_request::Verify>().await;
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
    let user = DBUser::fetch_user(&payload.name).await;

    match user {
        Ok(user) => match user {
            Some(user) => {
                if !verify(&payload.pin, &user.pin).unwrap() {
                    res.status_code(StatusCode::CREATED);
                    logger::log(logger::Actions::Verification { user: payload.name }, false).await;
                    increment_failed_attempts(req.remote_addr().to_owned()).await;
                    return res.render("failed to verify account");
                }
                unlock(req.remote_addr().to_owned()).await;
                res.status_code(StatusCode::OK);
                res.render("account verified");
                logger::log(logger::Actions::Verification { user: payload.name }, true).await;
                return;
            }
            None => {
                res.status_code(StatusCode::CREATED);
                res.render("account does not exist");
            }
        },
        Err(_) => {
            res.status_code(StatusCode::CREATED);
            res.render("Failed to connect to the database: error verifying account");
        }
    }
    logger::log(logger::Actions::Verification { user: payload.name }, false).await;
}
