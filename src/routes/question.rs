use std::collections::HashMap;
use warp::http::StatusCode;

use crate::error::Error;
use crate::store::Store;
use crate::types::pagination::extract_pagination;
use crate::types::question::{Question, QuestionId};

pub async fn get_questions(
    query_params: HashMap<String, String>,
    store: Store,
    id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("{} Will start querying questions", id);
    let res: Vec<Question> = store.questions.read().await.values().cloned().collect();

    if !query_params.is_empty() {
        let limit: usize = store.questions.read().await.len();
        let pagination = extract_pagination(query_params, limit)?;
        log::info!("{} Pagination set to {}", id, &pagination);
        let res = &res[pagination.start..pagination.end];
        return Ok(warp::reply::json(&res));
    }
    log::info!("{} No pagination used", id);
    Ok(warp::reply::json(&res))
}

pub async fn add_question(
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);

    Ok(warp::reply::with_status(
        "Question added successfully",
        StatusCode::OK,
    ))
}

pub async fn update_question(
    question_id: String,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store
        .questions
        .write()
        .await
        .get_mut(&QuestionId(question_id))
    {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

pub async fn remove_question(
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}
