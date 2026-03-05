use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct Contact {
    pub id: u32,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
}