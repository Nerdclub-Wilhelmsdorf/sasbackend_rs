use salvo::prelude::*;

#[handler]
pub async fn pay() -> &'static str {
    "Pay"
}
