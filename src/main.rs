use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::filters::cors::CorsForbidden;
use warp::reject::Reject;
use warp::{http::Method, http::StatusCode, Filter, Rejection, Reply};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Hash)]
struct QuestionId(String);

#[derive(Clone)]
struct Store {
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Self::init(),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("Can't read questions.json")
    }
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
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
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[derive(Debug)]
enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    InvertedOrder,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => {
                write!(f, "Can't parse parameter: {}", err)
            }
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::InvertedOrder => write!(f, "'start' can't be greater than 'end'"),
        }
    }
}

impl Reject for Error {}

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

fn extract_pagination(params: HashMap<String, String>, limit: usize) -> Result<Pagination, Error> {
    if !params.contains_key("start") || !params.contains_key("end") {
        return Err(Error::MissingParameters);
    }

    let start: usize = params
        .get("start")
        .unwrap()
        .parse::<usize>()
        .map_err(Error::ParseError)?;
    let mut end: usize = params
        .get("end")
        .unwrap()
        .parse::<usize>()
        .map_err(Error::ParseError)?;

    if start > end {
        return Err(Error::InvertedOrder);
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
    let limit: usize = store.questions.len();

    if !query_params.is_empty() {
        let pagination = extract_pagination(query_params, limit)?;
        let res: Vec<Question> = store.questions.values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store.questions.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
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
        .and(store_filter)
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items.with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
