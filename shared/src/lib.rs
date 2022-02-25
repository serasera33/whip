pub mod user;
pub mod roles;
pub mod venture;
pub mod goals;
pub mod tasks;
pub mod acl;

use moonlight::*;
use crate::user::LoginAttempt;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub enum UpMsg {
    Log(Message),
    Auth(LoginAttempt)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub enum DownMsg {
    Log(Message),
    AuthSuccess(String),
    AuthDenied
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "serde")]
pub struct Message {
    pub text: String,
}

impl Message {
    pub fn new(msg: &str) -> Message {
        Message {
            text: String::from(msg)
        }
    }
}