use moonlight::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "serde")]
pub enum Level {
    Junior,
    Senior
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "serde")]
pub enum Role {
    Frontend,
    Backend,
    UI,
    PM,
}