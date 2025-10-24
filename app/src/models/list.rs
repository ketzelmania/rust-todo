pub mod items;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct List {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
}
