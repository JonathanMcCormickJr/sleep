use std::sync::{Arc, Mutex};

use crate::models::*;
use crate::temp_state::TempState;
use axum::{
    extract::Extension,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use uuid::Uuid;

type AppState = Arc<Mutex<TempState>>;

// ---- CRUD for Questions ----

pub async fn create_question(
    Extension(state): Extension<AppState>,
    Json(question): Json<Question>,
) -> impl IntoResponse {
    let question_detail = QuestionDetail {
        question_uuid: Uuid::new_v4().to_string(),
        title: question.title,
        description: question.description,
        created_at: Utc::now().to_rfc3339(),
    };
    state.lock().unwrap().add_question(question_detail.clone());
    Json(question_detail)
}

pub async fn read_questions(
    Extension(state): Extension<AppState>,
) -> impl IntoResponse {
    Json(state.lock().unwrap().get_questions().clone())
}

pub async fn delete_question(
    Extension(state): Extension<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> Json<QuestionId> {
    let uuid = state
        .lock()
        .unwrap()
        .delete_question(&question_uuid.question_uuid);
    Json(QuestionId {
        question_uuid: uuid.to_string(),
    })
}

// ---- CRUD for Answers ----

pub async fn create_answer(
    Extension(state): Extension<AppState>,
    Json(answer): Json<Answer>,
) -> impl IntoResponse {
    let answer_detail = AnswerDetail {
        answer_uuid: Uuid::new_v4().to_string(),
        question_uuid: answer.question_uuid,
        content: answer.content,
        created_at: Utc::now().to_rfc3339(),
    };
    state.lock().unwrap().add_answer(answer_detail.clone());
    Json(answer_detail)
}

pub async fn read_answers(
    Extension(state): Extension<AppState>,
    Json(question_id): Json<QuestionId>,
) -> impl IntoResponse {
    let answers = state
        .lock()
        .unwrap()
        .get_answers(&question_id.question_uuid);
    match answers {
        Some(ans) => Json(ans.clone()),
        None => Json(Vec::new()),
    }
}

pub async fn delete_answer(
    Extension(state): Extension<AppState>,
    Json(answer_id): Json<AnswerId>,
) -> () {
    let _uuid = state.lock().unwrap().delete_answer(&answer_id.answer_uuid);
    //Json(AnswerId { answer_uuid: uuid.to_string() })
}
