use std::net::SocketAddr;

use axum::{
    routing::{delete, get, post},
    Router,
};

mod handlers;
mod models;
mod temp_state;

use handlers::*;
use temp_state::TempState;

#[tokio::main]
async fn main() {
    let mut state  = TempState::new(); // TODO: replace with persistent storage later
    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}