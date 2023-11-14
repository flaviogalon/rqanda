use serde::{Deserialize, Serialize};

use crate::types::question::QuestionId;

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Hash)]
pub struct AnswerId(pub i32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAnswer {
    pub content: String,
    pub question_id: QuestionId,
}
