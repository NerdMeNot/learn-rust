use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
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

// Shared by every request. Arc = many owners, Mutex = one writer at a time.
#[derive(Clone, Default)]
struct AppState {
    notes: Arc<Mutex<Vec<Note>>>,
}

async fn list(State(state): State<AppState>) -> Json<Vec<Note>> {
    let notes = state.notes.lock().unwrap();
    Json(notes.clone()) // the response owns a copy, so it does not borrow from the lock
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

async fn get_one(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Note>, StatusCode> {
    let notes = state.notes.lock().unwrap();
    match notes.iter().find(|n| n.id == id) {
        Some(note) => Ok(Json(note.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/notes", get(list).post(create))
        .route("/notes/:id", get(get_one))
        .with_state(AppState::default());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
