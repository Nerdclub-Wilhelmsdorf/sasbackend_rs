use salvo::prelude::*;

mod pay_routes;

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
