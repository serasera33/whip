use moonlight::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "serde")]
pub enum Permission {
    Read,
    Write,
    ReadWrite,
}