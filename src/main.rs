use salvo::prelude::*;
const DBURL: &str = "127.0.0.1:8000";
const DBUSER: &str = "guffe";
const DBPASS: &str = "IE76qzUk0t78JGhTz";
const TAX_RATE: &str = "0.1";
const TAX_FACTOR: &str = "1.1";
mod pay_routes;
mod user;
#[handler]
async fn hello() -> &'static str {
    "0"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let root_route = Router::new().get(hello);
    let pay_route = Router::new().path("/pay").post(pay_routes::pay);
    let router = Router::new().push(root_route).push(pay_route);
    let acceptor = TcpListener::new("127.0.0.1:1312").bind().await;
    Server::new(acceptor).serve(router).await;
}
