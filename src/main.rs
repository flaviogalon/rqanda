use std::io::{Error, ErrorKind};
use std::str::FromStr;

use warp::Filter;

#[derive(Debug)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
#[derive(Debug)]
struct QuestionId(String);

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

#[tokio::main]
async fn main() {
    let id = QuestionId::from_str("1").unwrap();

    // Leaving this here for now
    let question = Question::new(
        id,
        "First question!".to_string(),
        "Content of the question".to_string(),
        Some(vec!["faq".to_string()]),
    );

    let hello = warp::get().map(|| format!("Hello, world!"));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
