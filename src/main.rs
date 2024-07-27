use std::time::Duration;

use lock_user::is_locked;
use salvo::conn::rustls::{Keycert, RustlsConfig};
use salvo::cors::{self, Cors};
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
const TOKEN: &str = "Bearer W_97xyk8G]]w";
const TAX_FACTOR: &str = "10";
static DB: once_cell::sync::Lazy<Surreal<Client>> = once_cell::sync::Lazy::new(Surreal::init);
mod balance_check;
mod get_logs;
mod lock_user;
mod logger;
mod pay;
mod router;
mod user;
mod verify_account;
#[handler]
async fn hello() -> &'static str {
    "0"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let cert = tokio::fs::read("../fullchain.pem").await.unwrap();
    let key = tokio::fs::read("../privkey.pem").await.unwrap();
    db_connect().await;
    let cors: cors::CorsHandler = Cors::new()
        .allow_origin(cors::Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .allow_headers(vec!["authorization", "content-type"])
        .into_handler();
    let router = router::get_router();
    let service = Service::new(router)
        .hoop(cors)
        .hoop(Logger::new())
        .hoop(authorization)
        .hoop(check_for_user_lock);
    let config = RustlsConfig::new(Keycert::new().cert(cert.as_slice()).key(key.as_slice()));
    let listener = TcpListener::new(("banking.saswdorf.de", 443)).rustls(config.clone());
    let acceptor = QuinnListener::new(config, ("banking.saswdorf.de", 443))
        .join(listener)
        .bind()
        .await;
    Server::new(acceptor).serve(service).await;
}

async fn db_connect() {
    loop {
        let con = DB.connect::<Ws>(DBURL).await;
        match con {
            Ok(_) => break,
            Err(e) => {
                println!("Failed to connect to the database: {}, retrying", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
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
    if req.method() != Method::OPTIONS {
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
}

#[handler]
async fn check_for_user_lock(req: &mut Request, res: &mut Response) {
    if is_locked(req.remote_addr().to_owned()).await {
        res.status_code(StatusCode::TOO_MANY_REQUESTS);
        return res.render("suspended!");
    }
}
