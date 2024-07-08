use process_payment::PaymentRequest;
use salvo::prelude::*;
mod log_transaction;
mod payment_errors;
mod process_payment;
use crate::logger::log;

use self::process_payment::process_payment;

//desirialize the request   DONE
//fetch the sender and the receiver DONE
//check if the sender is locked WIP DELAYED
//check if the senders password is correct DONE
//check if the sender has enough money DONE
//subtract the sum from the sender (multiply by tax factor) //DONE
//add the sum to the receiver (multiply by tax factor)  //DONE
//add the tax to the bank //DONE
//remove the "failed attempts" status from the sender //WIP DELAYED
//log the transaction for all the parties //DONE
//return "success" //DONE
//if any of the checks fail, return "failed" //DONE

#[handler]
pub async fn pay(req: &mut Request, res: &mut Response) {
    let payload_result = req.parse_json::<PaymentRequest>().await;
    let payload = match payload_result {
        Ok(payload) => payload,
        Err(_) => {
            res.status_code(StatusCode::CREATED);
            return res.render("Failed to parse the request, are the values set according to the API documentation?");
        }
    };
    let mut payload = match payload.validate() {
        Some(e) => {
            res.status_code(StatusCode::CREATED);
            return res.render(e);
        }
        None => payload,
    };
    payload.amount = payload.amount.trim_start_matches('0').to_string();  
    let payment = process_payment(&payload).await;
    match payment {
        Ok(payment) => match payment {
            Ok(_) => {
                res.status_code(StatusCode::OK);
                res.render("success");
                log(
                    crate::logger::Actions::Transaction {
                        from: payload.from,
                        to: payload.to,
                        amount: payload.amount,
                    },
                    true,
                )
                .await;
                return;
            }
            Err(e) => {
                res.status_code(StatusCode::CREATED);
                res.render(e.to_string());
            }
        },
        Err(_) => {
            res.status_code(StatusCode::CREATED);
            res.render("Failed to connect to the database");
        }
    }
    log(
        crate::logger::Actions::Transaction {
            from: payload.from,
            to: payload.to,
            amount: payload.amount,
        },
        false,
    )
    .await;
}
