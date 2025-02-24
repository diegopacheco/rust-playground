use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cat {
    pub name: String,
    pub cat_type: String,
    pub fun_fact: String,
}