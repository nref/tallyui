use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Table {
    pub rows: HashMap<String, i32>,
}
