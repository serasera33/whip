use moonlight::*;
use crate::user::User;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "serde")]
pub struct Venture {
    id: String,
    name: String,
    users: Vec<User>
}