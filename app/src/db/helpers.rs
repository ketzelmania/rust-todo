use crate::models::items::TodoItem;
use crate::models::lists::List;
use crate::models::users::User;
use crate::schemas;
use sqlx::Error;

pub async fn new_user(env: &schemas::Environment, user: User) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO users (username, password) VALUES ($1, $2)",
        user.username,
        user.password,
    )
    .execute(env.db())
    .await?;

    Ok(())
}

pub async fn new_list(env: &schemas::Environment, list: List) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO lists (user_id, title) VALUES ($1, $2)",
        list.user_id,
        list.title,
    )
    .execute(env.db())
    .await?;

    Ok(())
}

pub async fn new_item(env: &schemas::Environment, item: &TodoItem) -> Result<(), Error> {
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

pub async fn get_user_from_id(env: &schemas::Environment, id: i32) -> Result<User, Error> {
    let list = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(env.db())
        .await?;

    Ok(list)
}

pub async fn get_list_from_id(env: &schemas::Environment, id: i32) -> Result<List, Error> {
    let list = sqlx::query_as!(List, "SELECT * FROM lists WHERE id = $1", id)
        .fetch_one(env.db())
        .await?;

    Ok(list)
}

pub async fn does_list_exist(env: &schemas::Environment, id: i32) -> Result<bool, Error> {
    get_list_from_id(env, id).await.and(Ok(true))
}

pub async fn get_list_items(env: &schemas::Environment, id: i32) -> Result<Vec<TodoItem>, Error> {
    let items = sqlx::query_as!(
            TodoItem,
            "SELECT items.id, items.list_id, items.title, items.status, items.description FROM items INNER JOIN lists ON items.list_id = lists.id WHERE lists.id = $1",
            id
        )
        .fetch_all(env.db())
        .await?;

    Ok(items)
}

#[cfg(test)]
mod test {
    use sqlx::PgPool;

    use super::*;
    use crate::db::test_helpers;

    #[sqlx::test]
    async fn test_get_user(pool: PgPool) {
        let env = schemas::Environment::from_pool(pool).await.unwrap();

        test_helpers::enter_users(&env).await.unwrap();

        assert_eq!(get_user_from_id(&env, 1).await.unwrap().username, "joe123");
    }

    #[sqlx::test]
    async fn test_new_user(pool: PgPool) {
        let env = schemas::Environment::from_pool(pool).await.unwrap();

        new_user(
            &env,
            User {
                id: 0,
                username: "bob".to_string(),
                password: "abc".to_string(),
                session_key: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(get_user_from_id(&env, 1).await.unwrap().username, "bob");
    }

    #[sqlx::test]
    async fn test_get_list(pool: PgPool) {
        let env = schemas::Environment::from_pool(pool).await.unwrap();

        test_helpers::enter_users(&env).await.unwrap();
        test_helpers::enter_lists(&env).await.unwrap();

        assert_eq!(
            get_list_from_id(&env, 1).await.unwrap().title,
            "joe123 list"
        );

        assert_eq!(
            get_list_from_id(&env, 6).await.unwrap().title,
            "another list"
        );
    }

    #[sqlx::test]
    async fn test_new_list(pool: PgPool) {
        let env = schemas::Environment::from_pool(pool).await.unwrap();

        test_helpers::enter_users(&env).await.unwrap();

        new_list(
            &env,
            List {
                id: 0,
                user_id: 1,
                title: "hello list".to_string(),
            },
        )
        .await
        .unwrap();

        assert_eq!(get_list_from_id(&env, 1).await.unwrap().title, "hello list");
    }

    #[sqlx::test]
    async fn test_get_items(pool: PgPool) {
        let env = schemas::Environment::from_pool(pool).await.unwrap();

        test_helpers::enter_users(&env).await.unwrap();
        test_helpers::enter_lists(&env).await.unwrap();
        test_helpers::enter_items(&env).await.unwrap();

        assert_eq!(
            get_list_items(&env, 3).await.unwrap(),
            vec![TodoItem {
                list_id: 3,
                id: 7,
                title: "only task".into(),
                status: Some("only task".into()),
                description: None,
            }]
        );
    }

    #[sqlx::test]
    async fn test_new_item(pool: PgPool) {
        let env = schemas::Environment::from_pool(pool).await.unwrap();

        test_helpers::enter_users(&env).await.unwrap();
        test_helpers::enter_lists(&env).await.unwrap();

        new_item(
            &env,
            &TodoItem {
                id: 0,
                list_id: 5,
                title: "hello task".into(),
                status: Some("hello status".into()),
                description: Some("hello description".into()),
            },
        )
        .await
        .unwrap();

        assert_eq!(
            get_list_items(&env, 5).await.unwrap(),
            vec![TodoItem {
                id: 1,
                list_id: 5,
                title: "hello task".into(),
                status: Some("hello status".into()),
                description: Some("hello description".into()),
            }]
        );
    }
}
