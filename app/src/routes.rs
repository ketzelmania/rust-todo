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
    warp::path!("lists")
        .and(warp::get())
        .and(warp::any().map(move || env.clone()))
        .and(warp::query::<requests::GetArgs>())
        //.and(warp::cookie::<String>("session_key")) // TODO: ts
        .and_then(handlers::get_list)
}

/// POST /lists?id=1234
fn post_list(env: Environment) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("lists")
        .and(warp::post())
        .and(warp::any().map(move || env.clone()))
        .and(warp::query::<List>())
        .and_then(handlers::post_list)
}

#[cfg(test)]
mod test {
    #[test]
    fn name() {
        todo!();
    }
}
