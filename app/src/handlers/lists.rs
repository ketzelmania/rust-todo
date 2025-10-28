use crate::db::helpers;
use crate::models::lists::List;
use crate::schemas::environment::Environment;
use crate::schemas::requests;
use warp::Reply;
use warp::reject::Rejection;

pub async fn get_list(env: Environment, args: requests::GetArgs) -> Result<impl Reply, Rejection> {
    let Ok(list) = helpers::get_list_from_id(&env, args.id).await else {
        return Err(warp::reject::not_found());
    };

    Ok(warp::reply::json(&list))
}

pub async fn post_list(env: Environment, args: List) -> Result<impl Reply, Rejection> {
    match helpers::new_list(&env, args).await {
        Ok(_) => Ok(warp::http::StatusCode::CREATED),
        Err(err) => {
            println!("{}", err);
            Err(warp::reject())
        }
    }
}
