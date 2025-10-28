use crate::models::list::List;
use crate::schemas::requests;
use crate::{handlers, schemas::environment::Environment};
use warp::{Filter, Rejection, Reply};

/// Combined routes for Lists
pub fn routes(env: Environment) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_list(env.clone()).or(post_list(env.clone()))
}

/// GET /lists?id=1234
fn get_list(env: Environment) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("lists")
        .and(warp::get())
        .and(warp::any().map(move || env.clone()))
        .and(warp::query::<requests::GetArgs>())
        //.and(warp::cookie::<String>("session_key")) // TODO: ts
        .and_then(handlers::get_list)
}

/// POST /lists?id=1234
fn post_list(env: Environment) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("lists")
        .and(warp::post())
        .and(warp::any().map(move || env.clone()))
        .and(warp::query::<List>())
        .and_then(handlers::post_list)
}

#[cfg(test)]
mod test {
    use crate::db::test_helpers;

    use super::*;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_get_list_success(pool: PgPool) {
        let env = Environment::from_pool(pool).await.unwrap();
        test_helpers::enter_users(&env).await.unwrap();
        test_helpers::enter_lists(&env).await.unwrap();
        test_helpers::enter_items(&env).await.unwrap();

        let filter = get_list(env);

        let res = warp::test::request()
            .method("GET")
            .path("/lists?id=1")
            .reply(&filter)
            .await;
        assert_eq!(res.status(), 200);
    }

    #[sqlx::test]
    async fn test_get_list_not_found(pool: PgPool) {
        let env = Environment::from_pool(pool).await.unwrap();
        test_helpers::enter_users(&env).await.unwrap();
        test_helpers::enter_lists(&env).await.unwrap();
        test_helpers::enter_items(&env).await.unwrap();

        let filter = get_list(env);

        let res = warp::test::request()
            .method("GET")
            .path("/lists?id=69")
            .reply(&filter)
            .await;
        assert_eq!(res.status(), 404);
    }

    #[sqlx::test]
    async fn test_get_list_invalid_params(pool: PgPool) {
        let env = Environment::from_pool(pool).await.unwrap();
        test_helpers::enter_users(&env).await.unwrap();
        test_helpers::enter_lists(&env).await.unwrap();
        test_helpers::enter_items(&env).await.unwrap();

        let filter = get_list(env);

        let res = warp::test::request()
            .method("GET")
            .path("/lists?joe=69")
            .reply(&filter)
            .await;
        assert_eq!(res.status(), 400);
    }

    #[sqlx::test]
    async fn test_post_list_success(pool: PgPool) {
        let env = Environment::from_pool(pool).await.unwrap();
        test_helpers::enter_users(&env).await.unwrap();

        let filter = post_list(env);

        let res = warp::test::request()
            .method("POST")
            .path("/lists?id=1&user_id=1&title=hello")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), 201);
    }

    #[sqlx::test]
    async fn test_post_list_invalid_user(pool: PgPool) {
        let env = Environment::from_pool(pool).await.unwrap();
        test_helpers::enter_users(&env).await.unwrap();

        let filter = post_list(env);

        let res = warp::test::request()
            .method("POST")
            .path("/lists?id=1&user_id=69&title=hello")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), 404);
    }

    #[sqlx::test]
    async fn test_post_list_missing_params(pool: PgPool) {
        let env = Environment::from_pool(pool).await.unwrap();
        test_helpers::enter_users(&env).await.unwrap();

        let filter = post_list(env);

        let res = warp::test::request()
            .method("POST")
            .path("/lists?id=1&user_id=1")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), 400);
    }

    #[sqlx::test]
    async fn test_post_list_invalid_params(pool: PgPool) {
        let env = Environment::from_pool(pool).await.unwrap();
        test_helpers::enter_users(&env).await.unwrap();

        let filter = post_list(env);

        let res = warp::test::request()
            .method("POST")
            .path("/lists?id=1&user_id=1&joe=18")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), 400);
    }
}
