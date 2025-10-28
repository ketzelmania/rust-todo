use crate::schemas;
use warp::{Filter, Rejection, Reply};

mod items;
mod lists;
mod users;

/// Combined routes for Lists
pub fn routes(
    env: schemas::Environment,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    lists::get_list(env.clone())
        .or(lists::post_list(env.clone()))
        .or(users::get_user(env.clone()))
        .or(users::post_user(env.clone()))
}
