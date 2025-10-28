use crate::db::helpers;
use crate::models::users::User;
use crate::schemas;
use warp::Reply;
use warp::reject::Rejection;

pub async fn get_user(
    env: schemas::Environment,
    args: schemas::GetArgs,
) -> Result<impl Reply, Rejection> {
    let Ok(list) = helpers::get_user_from_id(&env, args.id).await else {
        return Err(warp::reject::not_found());
    };

    Ok(warp::reply::json(&list))
}

pub async fn post_user(env: schemas::Environment, args: User) -> Result<impl Reply, Rejection> {
    match helpers::new_user(&env, args).await {
        Ok(_) => Ok(warp::http::StatusCode::CREATED),
        Err(err) => {
            println!("{}", err);
            Err(warp::reject())
        }
    }
}
