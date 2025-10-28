use crate::models::users::User;
use crate::{handlers, schemas};
use warp::{Filter, Rejection, Reply};

/// GET /users?id=1234
pub fn get_user(
    env: schemas::Environment,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("users")
        .and(warp::get())
        .and(warp::any().map(move || env.clone()))
        .and(warp::query::<schemas::GetArgs>())
        .and_then(handlers::users::get_user)
}

/// POST /users?id=1234
pub fn post_user(
    env: schemas::Environment,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("users")
        .and(warp::post())
        .and(warp::any().map(move || env.clone()))
        .and(warp::body::json::<User>())
        .and_then(handlers::users::post_user)
}

#[cfg(test)]
mod test {
    use crate::db::test_helpers;

    use super::*;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_get_user_successful(pool: PgPool) {
        let env = schemas::Environment::from_pool(pool).await.unwrap();
        test_helpers::enter_users(&env).await.unwrap();

        let filter = get_user(env);

        let res = warp::test::request()
            .method("GET")
            .path("/users?id=1")
            .reply(&filter)
            .await;
        assert_eq!(res.status(), 200);
    }

    #[sqlx::test]
    async fn test_get_user_not_found(pool: PgPool) {
        let env = schemas::Environment::from_pool(pool).await.unwrap();
        test_helpers::enter_users(&env).await.unwrap();

        let filter = get_user(env);

        let res = warp::test::request()
            .method("GET")
            .path("/users?id=69")
            .reply(&filter)
            .await;
        assert_eq!(res.status(), 404);
    }

    #[sqlx::test]
    async fn test_get_user_invalid_params(pool: PgPool) {
        let env = schemas::Environment::from_pool(pool).await.unwrap();
        test_helpers::enter_users(&env).await.unwrap();

        let filter = get_user(env);

        let res = warp::test::request()
            .method("GET")
            .path("/users?joe=69")
            .reply(&filter)
            .await;
        assert_eq!(res.status(), 400);
    }

    #[sqlx::test]
    async fn test_post_user_success(pool: PgPool) {
        let env = schemas::Environment::from_pool(pool).await.unwrap();
        test_helpers::enter_users(&env).await.unwrap();

        let filter = post_user(env);

        let res = warp::test::request()
            .method("POST")
            .path("/users")
            .json(&User {
                id: 0,
                username: "joe".into(),
                password: "abc".into(),
                session_key: None,
            })
            .reply(&filter)
            .await;

        assert_eq!(res.status(), 201);
    }
}
