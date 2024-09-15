use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Song {
    pub id: u64,
    pub name: String,
}
