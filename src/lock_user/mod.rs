use std::sync::Arc;

use once_cell::sync::Lazy;
use salvo::conn::SocketAddr;
use tokio::sync::Mutex;

struct User {
    ip: SocketAddr,
    failed_attempts: u8,
}

struct Users {
    users: Vec<User>,
}

static TEST_VCTR: Lazy<Arc<Mutex<Users>>> =
    Lazy::new(|| Arc::new(Mutex::new(Users { users: Vec::new() })));

pub async fn increment_failed_attempts(ip: SocketAddr) {
    let mut users = TEST_VCTR.lock().await;
    let mut found = false;
    for user in users.users.iter_mut() {
        if matches!(&user.ip, ip) {
            user.failed_attempts += 1;
            if user.failed_attempts >= 3 {
                // start unlock timer
                tokio::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_secs(60 * 5)).await;
                    let mut users = TEST_VCTR.lock().await;
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
    let users = TEST_VCTR.lock().await;
    for user in users.users.iter() {
        if matches!(&user.ip, ip) {
            return user.failed_attempts >= 3;
        }
    }
    false
}

pub async fn unlock(ip: SocketAddr) {
    let mut users = TEST_VCTR.lock().await;
    users.users.retain(|u| !matches!(&u.ip, ip));
}
