use moonlight::*;
use crate::tasks::Task;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "serde")]
pub struct Goal {
    id: String,
    name: String,
    tasks: Vec<Task>
}