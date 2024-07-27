use std::sync::Arc;

use tokio::sync::{RwLock};

use once_cell::sync::Lazy;
use salvo::conn::SocketAddr;

struct User {
    ip: SocketAddr,
    failed_attempts: u8,
}

struct Users {
    users: Vec<User>,
}

static LOCKED_USERS: Lazy<Arc<RwLock<Users>>> =
    Lazy::new(|| Arc::new(RwLock::new(Users { users: Vec::new() })));

pub async fn increment_failed_attempts(ip: SocketAddr) {
    let mut users = LOCKED_USERS.write().await;
    let mut found = false;
    for user in users.users.iter_mut() {
        if matches!(&user.ip, ip) {
            user.failed_attempts += 1;
            if user.failed_attempts >= 3 {
                // start unlock timer
                tokio::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_secs(60 * 15)).await;
                    let mut users = LOCKED_USERS.write().await;
                    users.users.retain(|u| !matches!(&u.ip, ip));
                });
            }
            found = true;
            break;
        }
    }
    if !found {
        users.users.push(User {
            ip,
            failed_attempts: 1,
        });
    }
}

pub async fn is_locked(ip: SocketAddr) -> bool {
    let users = LOCKED_USERS.read().await;
    for user in users.users.iter() {
        if matches!(&user.ip, ip) {
            return user.failed_attempts >= 3;
        }
    }
    false
}

pub async fn unlock(ip: SocketAddr) {
    let mut users = LOCKED_USERS.write().await;
    users.users.retain(|u| !matches!(&u.ip, ip));
}
