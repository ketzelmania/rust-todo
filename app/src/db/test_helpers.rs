use crate::schemas;
use sqlx::Error;

pub async fn setup_test_db(env: &schemas::Environment) -> Result<(), Error> {
    sqlx::migrate!().run(env.db()).await?;

    Ok(())
}

pub async fn enter_users(env: &schemas::Environment) -> Result<(), Error> {
    let users = [
        "joe123", "joe124", "joe125", "joe126", "joe127", "joe128", "joe129", "joe130", "joe131",
    ];

    let passwords = [
        "abc", "abc", "abc", "abc", "abc", "abc", "abc", "abc", "abc",
    ];

    for i in 0..users.len() {
        sqlx::query!(
            "INSERT INTO users (username, password) VALUES ($1, $2)",
            users[i],
            passwords[i],
        )
        .execute(env.db())
        .await?;
    }

    Ok(())
}

pub async fn enter_lists(env: &schemas::Environment) -> Result<(), Error> {
    let user_ids = [1, 1, 1, 2, 2, 3];
    let titles = [
        "joe123 list",
        "another list",
        "joe list",
        "joe124 list",
        "test list",
        "another list",
    ];

    for i in 0..titles.len() {
        sqlx::query!(
            "INSERT INTO lists (title, user_id) VALUES ($1, $2)",
            titles[i],
            user_ids[i],
        )
        .execute(env.db())
        .await?;
    }

    Ok(())
}

pub async fn enter_items(env: &schemas::Environment) -> Result<(), Error> {
    let list_ids = [1, 1, 1, 1, 2, 2, 3, 4];
    let titles = [
        "do thing 1",
        "do thing 2",
        "merge things",
        "destroy things",
        "task A",
        "task B",
        "only task",
        "another only task",
    ];
    let statuses = [
        Some("complete"),
        Some("incomplete"),
        None,
        None,
        Some("task A"),
        Some("task B"),
        Some("only task"),
        Some("another only task"),
    ];
    let descriptions = [
        Some("process the thing"),
        None,
        Some("needs to be done asap"),
        None,
        None,
        None,
        None,
        None,
    ];

    for i in 0..titles.len() {
        sqlx::query!(
            "INSERT INTO items (list_id, title, status, description) VALUES ($1, $2, $3, $4)",
            list_ids[i],
            titles[i],
            statuses[i],
            descriptions[i]
        )
        .execute(env.db())
        .await?;
    }

    Ok(())
}
