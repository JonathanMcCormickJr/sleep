use std::sync::{Arc, Mutex};

use crate::models::*;
use axum::{extract, response::IntoResponse, Json};
use chrono::Utc;
use uuid::Uuid;

// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>, state: extract::Extension<Arc<Mutex<crate::temp_state::TempState>>>) -> impl IntoResponse {
    let question_detail = QuestionDetail {
        question_uuid: Uuid::new_v4().to_string(),
        title: question.title,
        description: question.description,
        created_at: Utc::now().to_rfc3339(),
    };
    state.lock().unwrap().add_question(question_detail.clone());
    Json(question_detail)
}

pub async fn read_questions(state: extract::Extension<Arc<Mutex<crate::temp_state::TempState>>>) -> impl IntoResponse {
    Json(state.lock().unwrap().get_questions().clone())
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>, state: extract::Extension<Arc<Mutex<crate::temp_state::TempState>>>) -> Json<QuestionId> {
    let uuid = state.lock().unwrap().delete_question(&question_uuid.question_uuid);
    Json(QuestionId { question_uuid: uuid.to_string() })
}

// ---- CRUD for Answers ----
// CONTINUE HERE

// TODO: Create a POST route to /answer which accepts an `Answer` and returns `AnswerDetail` as JSON.
//       The handler function should be called `create_answer`.
//
//       hint: this function should look very similar to the create_question function above

// TODO: Create a GET route to /answers which accepts an `QuestionId` and returns a vector of `AnswerDetail` as JSON.
//       The handler function should be called `read_answers`.
//
//       hint: this function should look very similar to the read_questions function above

// TODO: Create a DELETE route to /answer which accepts an `AnswerId` and does not return anything.
//       The handler function should be called `delete_answer`.
//
//       hint: this function should look very similar to the delete_question function above