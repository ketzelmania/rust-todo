use crate::models::list::RawList;
use crate::schemas::environment::Environment;
use crate::schemas::requests;
use warp::Reply;
use warp::reject::Rejection;

pub async fn get_list(env: Environment, args: requests::GetArgs) -> Result<impl Reply, Rejection> {
    let Ok(list) = sqlx::query_as!(RawList, "SELECT * FROM lists WHERE id = $1", args.id)
        .fetch_one(env.db())
        .await
    else {
        return Err(warp::reject());
    };

    Ok(warp::reply::json(&list))
}

// TODO: this fn
pub async fn post_list(
    env: Environment,
    args: requests::PostArgs,
) -> Result<impl Reply, Rejection> {
    let Ok(list) = sqlx::query_as!(RawList, "SELECT * FROM lists WHERE id = $1", args.list_id)
        .fetch_one(env.db())
        .await
    else {
        return Err(warp::reject());
    };

    Ok(warp::reply::json(&list))
}
