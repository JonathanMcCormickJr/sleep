use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub title: String,
    pub description: String,
}

/// The base structure representing a detailed view of a question.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionDetail {
    pub question_uuid: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Deserialize, Serialize)]
pub struct QuestionId {
    pub question_uuid: String,
}

#[derive(Deserialize, Serialize)]
pub struct Answer {
    pub question_uuid: String,
    pub content: String,
}

/// The base structure representing a detailed view of an answer.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnswerDetail {
    pub answer_uuid: String,
    pub question_uuid: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Deserialize, Serialize)]
pub struct AnswerId {
    pub answer_uuid: String,
}
