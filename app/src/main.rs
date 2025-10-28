use dotenvy::dotenv;

mod db;
mod handlers;
mod models;
mod routes;
mod schemas;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let env = schemas::Environment::new(&url)
        .await
        .expect("Failed to parse env");

    println!("Database connection successful");

    let routes = routes::routes(env);

    println!("Server started");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
