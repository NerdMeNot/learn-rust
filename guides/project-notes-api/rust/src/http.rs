//! The HTTP layer: routes, JSON, and the mapping from errors to status codes.
//! It is written against `NoteStore` and never mentions SQLite.

use crate::domain::{InvalidText, Note, NoteText};
use crate::storage::{NoteStore, StoreError};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::Deserialize;
use serde_json::json;
use std::error::Error;
use std::sync::Arc;

pub fn router<S: NoteStore>(store: S) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/notes", get(list::<S>).post(create::<S>))
        .route("/notes/{id}", get(get_one::<S>).delete(remove::<S>))
        .with_state(Arc::new(store))
}

#[derive(Deserialize)]
struct CreateNote {
    text: String,
}

async fn health() -> &'static str {
    "ok"
}

async fn list<S: NoteStore>(State(store): State<Arc<S>>) -> Result<Json<Vec<Note>>, ApiError> {
    Ok(Json(store.list().await?))
}

async fn create<S: NoteStore>(
    State(store): State<Arc<S>>,
    Json(body): Json<CreateNote>,
) -> Result<(StatusCode, Json<Note>), ApiError> {
    let text = NoteText::parse(&body.text)?;
    let note = store.insert(&text).await?;
    tracing::info!(id = note.id, "note created");
    Ok((StatusCode::CREATED, Json(note)))
}

async fn get_one<S: NoteStore>(
    State(store): State<Arc<S>>,
    Path(id): Path<i64>,
) -> Result<Json<Note>, ApiError> {
    store.get(id).await?.map(Json).ok_or(ApiError::NotFound(id))
}

async fn remove<S: NoteStore>(
    State(store): State<Arc<S>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    if store.delete(id).await? {
        tracing::info!(id, "note deleted");
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotFound(id))
    }
}

/// Everything a handler can fail with, and the one place that turns it into a response.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error(transparent)]
    Invalid(#[from] InvalidText),
    #[error("no note with id {0}")]
    NotFound(i64),
    #[error(transparent)]
    Store(#[from] StoreError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::Invalid(e) => (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()),
            ApiError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::Store(e) => {
                // The details go to the log, not to the caller.
                tracing::error!(error = %e, cause = ?e.source(), "storage failed");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal error".to_string(),
                )
            }
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
