mod auth;
mod db;

use crate::auth::check_password;
use log::{debug, error, info, log_enabled, warn, Level, LevelFilter};
use moon::{start, Frontend, UpMsgRequest, *};
use shared::{DownMsg, Message, UpMsg};
use sqlx::postgres::PgPoolOptions;
use crate::db::db_connection;
async fn frontend() -> Frontend {
    Frontend::new().title("Whip").default_styles(false)
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "backend=debug");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://whip:whip@localhost/whip").await.expect("Failed to connect");

    db_connection().set(Some(pool));
    start(frontend, up_msg_handler, |_| {}).await
}

async fn up_msg_handler(req: UpMsgRequest<UpMsg>) {
    debug!("Up Message is {:?}", req);
    let UpMsgRequest {
        up_msg,
        cor_id,
        session_id,
        auth_token,
    } = req;
    let down_msg: DownMsg;
    match up_msg {
        UpMsg::Log(message) => {
            info!("Received {:?}", message);
        }
        UpMsg::Auth(login) => {
            let auth_attempt = check_password(login).await;
            match auth_attempt {
                Some(authkey) => {
                    down_msg = DownMsg::AuthSuccess(AuthToken::new(authkey).into_string());
                    if let Some(session) = sessions::by_session_id().wait_for(session_id).await {
                        session.send_down_msg(&down_msg, cor_id).await;
                        info!("{} Sent AuthKey back {:?}", session_id, down_msg);
                    } else {
                        error!("cannot find the session with id `{}`", session_id);
                    }
                }
                None => {
                    down_msg = DownMsg::AuthDenied;
                    if let Some(session) = sessions::by_session_id().wait_for(session_id).await {
                        session.send_down_msg(&down_msg, cor_id).await;
                        warn!("{} Authentication denied", session_id);
                    } else {
                        error!("cannot find the session with id `{}`", session_id);
                    }
                }
            }
        }
    }
}
