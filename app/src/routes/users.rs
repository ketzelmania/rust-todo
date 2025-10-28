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
