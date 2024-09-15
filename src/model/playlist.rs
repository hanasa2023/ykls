use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Playlist {
    pub id: u64,
    pub name: String,
}
