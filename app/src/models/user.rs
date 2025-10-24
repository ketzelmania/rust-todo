#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub session_key: Option<String>,
}
