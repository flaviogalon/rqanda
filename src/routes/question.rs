use std::collections::HashMap;
use warp::http::StatusCode;

use tracing::{error, info, instrument};

use crate::get_bad_words_api_key;
use crate::store::Store;
use crate::types::pagination::{extract_pagination, Pagination};
use crate::types::question::{NewQuestion, Question, UpdateQuestion};

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
    // Calling bad words api here for now
    let client = reqwest::Client::new();
    // Making the request in the unsafest way possible just for fun
    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", get_bad_words_api_key())
        .body("a list with shit words")
        .send()
        .await
        .expect("Error calling external API")
        .text()
        .await
        .expect("Error parsing response from external API");

    println!("{}", res);

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

pub async fn remove_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.remove_question(id).await {
        Ok(true) => Ok(warp::reply::with_status("", StatusCode::OK)),
        Ok(false) => Ok(warp::reply::with_status("", StatusCode::NOT_FOUND)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
