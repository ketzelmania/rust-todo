use serde;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TodoItem {
    pub status: String,
    pub title: String,
    pub description: String,
}
