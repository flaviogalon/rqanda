use std::io::{Error, ErrorKind};
use std::str::FromStr;

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

fn main() {
    let id = QuestionId::from_str("1").unwrap();

    let question = Question::new(
        id,
        "First question!".to_string(),
        "Content of the question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    println!("{:?}", question);
}
