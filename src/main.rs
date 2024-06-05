use salvo::conn::rustls::{Keycert, RustlsConfig};
use salvo::cors::Cors;
use salvo::http::Method;
use salvo::prelude::*;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
const DBURL: &str = "127.0.0.1:8000";
const DBUSER: &str = "guffe";
const DBPASS: &str = "IE76qzUk0t78JGhTz";
const TOKEN: &str = "Bearer test";
const TAX_FACTOR: &str = "1.1";
static DB: once_cell::sync::Lazy<Surreal<Client>> = once_cell::sync::Lazy::new(Surreal::init);
mod balance_check;
mod get_logs;
mod pay;
mod user;
mod verify_account;
#[handler]
async fn hello() -> &'static str {
    "0"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let cert = include_bytes!("fullchain.pem").to_vec();
    let key = include_bytes!("privkey.pem").to_vec();
    db_connect().await;
    let cors = Cors::new()
        .allow_origin("*")
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .into_handler();

    let root_route = Router::new().get(hello);
    let pay_route = Router::new().path("/pay").post(pay::pay);
    let balance_route = Router::new()
        .path("/balanceCheck")
        .post(balance_check::balance_check);
    let log_route = Router::new().path("/getLogs").post(get_logs::get_logs);
    let verify_route = Router::new()
        .path("/verify")
        .post(verify_account::verify_account);
    let router = Router::new()
        .push(root_route)
        .push(pay_route)
        .push(log_route)
        .push(verify_route)
        .push(balance_route);
    let service = Service::new(router).hoop(cors).hoop(Logger::new());
    let config = RustlsConfig::new(Keycert::new().cert(cert.as_slice()).key(key.as_slice()));
    let listener = TcpListener::new(("127.0.0.1", 8443)).rustls(config.clone());
    let acceptor = QuinnListener::new(config, ("127.0.0.1", 8443))
        .join(listener)
        .bind()
        .await;
    Server::new(acceptor).serve(service).await;
}

async fn db_connect() {
    DB.connect::<Ws>(DBURL).await.unwrap();
    DB.signin(Root {
        username: DBUSER,
        password: DBPASS,
    })
    .await
    .unwrap();
    // Select a namespace + database
    DB.use_ns("user").use_db("user").await.unwrap();
}

#[handler]
async fn authorization(req: &mut Request, res: &mut Response) {
    let auth = match req.headers().get("Authorization") {
        Some(auth) => auth,
        None => {
            res.status_code(StatusCode::UNAUTHORIZED);
            return res.render("No authorization header found");
        }
    };
    let auth = match auth.to_str() {
        Ok(auth) => auth,
        Err(_) => {
            res.status_code(StatusCode::UNAUTHORIZED);
            return res.render("Failed to parse the authorization header");
        }
    };
    if auth != TOKEN {
        res.status_code(StatusCode::UNAUTHORIZED);
        return res.render("Invalid token");
    }
}