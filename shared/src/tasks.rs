use moonlight::*;
use crate::user::User;
use crate::venture::Venture;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "serde")]
pub struct Task {
    id: String,
    name: String,
    ventures: Vec<Venture>,
    users: Vec<User>
}