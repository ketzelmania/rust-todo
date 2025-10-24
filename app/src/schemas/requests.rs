use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetArgs {
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostArgs {
    pub user_id: i32,
    pub list_id: i32,
    pub list_title: String,
}
