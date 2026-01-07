//! A home for temporary in-memory state management during development.

// TODO: Replace TempState with persistent storage solution later.

use uuid::Uuid;

pub struct TempState {
    pub questions: Vec<crate::models::QuestionDetail>,
    pub answers: std::collections::HashMap<Uuid, Vec<crate::models::AnswerDetail>>,
}

impl TempState {
    pub fn new() -> Self {
        Self {
            questions: Vec::new(),
            answers: std::collections::HashMap::new(),
        }
    }

    pub fn add_question(&mut self, question: crate::models::QuestionDetail) {
        self.questions.push(question);
    }

    pub fn get_questions(&self) -> &Vec<crate::models::QuestionDetail> {
        &self.questions
    }

    pub fn delete_question(&mut self, question_uuid: &str) -> Uuid {
        self.questions
            .retain(|q| q.question_uuid != question_uuid);
        let uuid = Uuid::parse_str(question_uuid).unwrap();
        self.answers
            .remove(&Uuid::parse_str(question_uuid).unwrap());
        uuid
    }
}