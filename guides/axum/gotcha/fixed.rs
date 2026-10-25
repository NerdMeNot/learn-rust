use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Serialize)]
struct Note {
    id: u32,
    text: String,
}

#[derive(Deserialize)]
struct NewNote {
    text: String,
}

#[derive(Clone, Default)]
struct AppState {
    notes: Arc<Mutex<Vec<Note>>>,
}

async fn create(
    State(state): State<AppState>,
    Json(input): Json<NewNote>, // the request body: it must be the last argument
) -> (StatusCode, Json<Note>) {
    let mut notes = state.notes.lock().unwrap();
    let note = Note {
        id: notes.len() as u32 + 1,
        text: input.text,
    };
    notes.push(note.clone());
    (StatusCode::CREATED, Json(note))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/notes", post(create))
        .with_state(AppState::default());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
