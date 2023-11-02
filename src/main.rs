use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};

mod error;
mod routes;
mod store;
mod types;

#[tokio::main]
async fn main() {
    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "rqanda=info,warp=error".to_owned());

    // Setting CORS policy in application level since we're serving a single instance
    // of the application without an infra-level on front.
    let cors = warp::cors()
        .allow_any_origin() // Not safe for production!
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let store = store::Store::new();
    let store_filter = warp::any().map(move || store.clone());

    tracing_subscriber::fmt()
        // Filter will determine which traces to record
        .with_env_filter(log_filter)
        // Event is recorded when span closes
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::question::get_questions)
        .with(warp::trace(|info| {
            tracing::info_span!("get_items request",
        method = %info.method(),
        path = %info.path(),
        id = %uuid::Uuid::new_v4())
        }));

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let remove_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::question::remove_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(routes::answer::add_answer);

    let routes = get_items
        .or(add_question)
        .or(add_answer)
        .or(update_question)
        .or(remove_question)
        .with(cors)
        .with(warp::trace::request())
        .recover(error::return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
