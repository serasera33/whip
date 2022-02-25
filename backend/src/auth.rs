use std::ops::Deref;

use log::{debug, info};
use moon::{rusty_ulid, Ulid};
use moon::actix_web::http::header::DispositionParam::Name;
use sqlx::{Execute, Row};

use shared::user::LoginAttempt;

use crate::db_connection;

pub async fn check_password(login_attempt: LoginAttempt) -> Option<String> {
    let mut lock = db_connection().lock_mut();
    let pool = lock.as_ref().unwrap();
    let email = login_attempt.email_id.clone();
    let auth_check_query =
        sqlx::query("SELECT id FROM users WHERE email = $1 and password = crypt($2, password)")
            .bind(login_attempt.email_id)
            .bind(login_attempt.password);
    debug!("{:?}", auth_check_query.sql());
    let auth_check = auth_check_query
        .fetch_optional(pool)
        .await
        .expect("could not query!!");

    match auth_check {
        Some(_) => {
            info!("Auth check successful for {}", email);
            Some(rusty_ulid::generate_ulid_string())
        }
        None => None,
    }
}
