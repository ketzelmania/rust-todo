use crate::models::list::List;
use crate::models::list::items::TodoItem;
use crate::models::user::User;
use crate::schemas::environment::Environment;
use sqlx::Error;

pub async fn new_user(env: &Environment, user: User) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO users (username, password) VALUES ($1, $2)",
        user.username,
        user.password,
    )
    .execute(env.db())
    .await?;

    Ok(())
}

pub async fn new_list(env: &Environment, list: List) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO lists (user_id, title) VALUES ($1, $2)",
        list.user_id,
        list.title,
    )
    .execute(env.db())
    .await?;

    Ok(())
}

pub async fn new_item(env: &Environment, item: &TodoItem) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO items (list_id, title, status, description) VALUES ($1, $2, $3, $4)",
        item.list_id,
        item.title,
        item.status,
        item.description,
    )
    .execute(env.db())
    .await?;

    Ok(())
}

// TODO: session keys

pub async fn get_list_from_id(env: &Environment, id: i32) -> Result<List, Error> {
    let list = sqlx::query_as!(List, "SELECT * FROM lists WHERE id = $1", id)
        .fetch_one(env.db())
        .await?;

    Ok(list)
}

pub async fn does_list_exist(env: &Environment, id: i32) -> Result<bool, Error> {
    get_list_from_id(env, id).await.and(Ok(true))
}

pub async fn get_list_items(env: &Environment, id: i32) -> Result<Vec<TodoItem>, Error> {
    let items = sqlx::query_as!(
            TodoItem,
            "SELECT items.id, items.list_id, items.title, items.status, items.description FROM items INNER JOIN lists ON items.list_id = lists.id WHERE lists.id = $1",
            id
        )
        .fetch_all(env.db())
        .await?;

    Ok(items)
}
