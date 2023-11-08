use std::collections::HashMap;
use warp::http::StatusCode;

use tracing::{info, instrument};

use crate::error::Error;
use crate::store::Store;
use crate::types::pagination::{self, extract_pagination, Pagination};
use crate::types::question::{NewQuestion, Question, QuestionId, UpdateQuestion};

#[instrument]
pub async fn get_questions(
    query_params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Will start querying questions");
    let mut pagination = Pagination::default();

    if !query_params.is_empty() {
        info!(pagination = true);
        pagination = extract_pagination(query_params)?;
    } else {
        info!(pagination = false);
    }
    let response: Vec<Question> = match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    Ok(warp::reply::json(&response))
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    let question = match store.add_question(new_question).await {
        Ok(question) => question,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&question),
        StatusCode::CREATED,
    ))
}

pub async fn update_question(
    question_id: i32,
    store: Store,
    update_question: UpdateQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    let updated_question = match store.update_question(update_question, question_id).await {
        Ok(question) => question,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&updated_question))
}

// pub async fn remove_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
//     match store.questions.write().await.remove(&QuestionId(id)) {
//         Some(_) => Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
//         None => Err(warp::reject::custom(Error::QuestionNotFound)),
//     }
// }
