use serde::{Deserialize, Serialize};

/// index model
#[derive(Debug, Deserialize, Serialize)]
pub struct Index {
    pub id: u32,
    pub name: String,
    pub thumbnail: Vec<u8>,
    pub description: String,
}
