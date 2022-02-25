use moonlight::*;
use crate::roles::{Level, Role};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "serde")]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email_id: String,
    pub role: Role,
    pub level: Level
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "serde")]
pub struct LoginAttempt {
    pub email_id: String,
    pub password: String
}

impl LoginAttempt {
    pub fn init() -> LoginAttempt {
        LoginAttempt {
            email_id: String::from(""),
            password: String::from(""),
        }
    }

    pub fn from(oldla: &LoginAttempt) -> LoginAttempt {
        LoginAttempt {
            email_id: oldla.email_id.clone(),
            password: oldla.password.clone()
        }
    }
}