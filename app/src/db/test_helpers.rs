use crate::schemas::environment::Environment;
use sqlx::Error;

pub async fn setup_test_db(env: &Environment) -> Result<(), Error> {
    reset_test_db(env).await?;

    sqlx::migrate!("db/migrations").run(env.db()).await?;

    Ok(())
}

pub async fn reset_test_db(env: &Environment) -> Result<(), Error> {
    sqlx::query!("DROP DATABASE \"rust-todo-test-db\"")
        .execute(env.db())
        .await?;

    sqlx::query!("CREATE DATABASE \"rust-todo-test-db\"")
        .execute(env.db())
        .await?;

    Ok(())
}

pub async fn enter_users(env: &Environment) -> Result<(), Error> {
    // sqlx::query!("INSERT INTO users () VALUES ()")
    //     .execute(env.db())
    //     .await?;

    Ok(())
}
