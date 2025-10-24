use serde;
use sqlx::Error;

pub mod items;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RawList {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct List {
    pub id: i32,
    pub title: String,
    pub items: Vec<items::TodoItem>,
}

impl List {
    pub async fn from_raw(raw: RawList) -> Result<List, Error> {
        Ok(List {
            id: 0,
            title: String::from("Title"),
            items: vec![],
        })
    }
}
