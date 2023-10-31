use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{http::Method, http::StatusCode, Filter};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Hash)]
struct QuestionId(String);

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Hash)]
struct AnswerId(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Answer {
    id: AnswerId,
    content: String,
    question_id: QuestionId,
}

#[derive(Clone)]
struct Store {
    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("Can't read questions.json")
    }
}

mod error {
    use warp::{
        filters::{body::BodyDeserializeError, cors::CorsForbidden},
        http::StatusCode,
        reject::Reject,
        Rejection, Reply,
    };

    #[derive(Debug)]
    pub enum Error {
        ParseError(std::num::ParseIntError),
        MissingParameters,
        InvertedOrder,
        QuestionNotFound,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match *self {
                Error::ParseError(ref err) => {
                    write!(f, "Can't parse parameter: {}", err)
                }
                Error::MissingParameters => write!(f, "Missing parameter"),
                Error::InvertedOrder => write!(f, "'start' can't be greater than 'end'"),
                Error::QuestionNotFound => write!(f, "Question not found in store"),
            }
        }
    }

    impl Reject for Error {}

    pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
        if let Some(error) = r.find::<CorsForbidden>() {
            Ok(warp::reply::with_status(
                error.to_string(),
                StatusCode::FORBIDDEN,
            ))
        } else if let Some(error) = r.find::<Error>() {
            Ok(warp::reply::with_status(
                error.to_string(),
                StatusCode::RANGE_NOT_SATISFIABLE,
            ))
        } else if let Some(error) = r.find::<BodyDeserializeError>() {
            Ok(warp::reply::with_status(
                error.to_string(),
                StatusCode::UNPROCESSABLE_ENTITY,
            ))
        } else {
            Ok(warp::reply::with_status(
                "Route not found".to_string(),
                StatusCode::NOT_FOUND,
            ))
        }
    }
}

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

fn extract_pagination(
    params: HashMap<String, String>,
    limit: usize,
) -> Result<Pagination, error::Error> {
    if !params.contains_key("start") || !params.contains_key("end") {
        return Err(error::Error::MissingParameters);
    }

    let start: usize = params
        .get("start")
        .unwrap()
        .parse::<usize>()
        .map_err(error::Error::ParseError)?;
    let mut end: usize = params
        .get("end")
        .unwrap()
        .parse::<usize>()
        .map_err(error::Error::ParseError)?;

    if start > end {
        return Err(error::Error::InvertedOrder);
    } else if end > limit {
        end = limit;
    }

    return Ok(Pagination {
        start: start,
        end: end,
    });
}

async fn get_questions(
    query_params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res: Vec<Question> = store.questions.read().await.values().cloned().collect();

    if !query_params.is_empty() {
        let limit: usize = store.questions.read().await.len();
        let pagination = extract_pagination(query_params, limit)?;
        let res = &res[pagination.start..pagination.end];
        return Ok(warp::reply::json(&res));
    }
    Ok(warp::reply::json(&res))
}

async fn add_question(
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

async fn update_question(
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
        None => return Err(warp::reject::custom(error::Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

async fn remove_question(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => return Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
        None => return Err(warp::reject::custom(error::Error::QuestionNotFound)),
    }
}

async fn add_answer(
    store: Store,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let answer = Answer {
        id: AnswerId("1".to_string()),
        content: params.get("content").unwrap().to_string(),
        question_id: QuestionId(params.get("questionId").unwrap().to_string()),
    };

    store
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);

    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}

#[tokio::main]
async fn main() {
    // Setting CORS policy in application level since we're serving a single instance
    // of the application without an infra-level on front.
    let cors = warp::cors()
        .allow_any_origin() // Not safe for production!
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_questions);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_question);

    let remove_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(remove_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(add_answer);

    let routes = get_items
        .or(add_question)
        .or(add_answer)
        .or(update_question)
        .or(remove_question)
        .with(cors)
        .recover(error::return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
