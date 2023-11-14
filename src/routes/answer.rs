use warp::http::StatusCode;

use crate::store::Store;
use crate::types::answer::NewAnswer;

pub async fn add_answer(
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    let answer = match store.add_answer(new_answer).await {
        Ok(answer) => answer,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&answer),
        StatusCode::CREATED,
    ))
}
