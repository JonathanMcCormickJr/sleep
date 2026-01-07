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

pub async fn create_answer(Json(answer): Json<Answer>, state: extract::Extension<Arc<Mutex<crate::temp_state::TempState>>>) -> impl IntoResponse {
    let answer_detail = AnswerDetail {
        answer_uuid: Uuid::new_v4().to_string(),
        question_uuid: answer.question_uuid,
        content: answer.content,
        created_at: Utc::now().to_rfc3339(),
    };
    state.lock().unwrap().add_answer(answer_detail.clone());
    Json(answer_detail)
}

pub async fn read_answers(extract::Json(question_id): extract::Json<QuestionId>, state: extract::Extension<Arc<Mutex<crate::temp_state::TempState>>>) -> impl IntoResponse {
    let answers = state.lock().unwrap().get_answers(&question_id.question_uuid);
    match answers {
        Some(ans) => Json(ans.clone()),
        None => Json(Vec::new()),
    }
}

// TODO: Create a DELETE route to /answer which accepts an `AnswerId` and does not return anything.
//       The handler function should be called `delete_answer`.
//
//       hint: this function should look very similar to the delete_question function above
pub async fn delete_answer(extract::Json(answer_id): extract::Json<AnswerId>, state: extract::Extension<Arc<Mutex<crate::temp_state::TempState>>>) -> () {
    let uuid = state.lock().unwrap().delete_answer(&answer_id.answer_uuid);
    //Json(AnswerId { answer_uuid: uuid.to_string() })
}