use serde;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TodoItem {
    pub id: i32,
    pub list_id: i32,
    pub title: String,
    pub status: Option<String>,
    pub description: Option<String>,
}

impl TodoItem {}
