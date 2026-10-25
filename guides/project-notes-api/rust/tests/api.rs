//! Integration tests: the whole router, called in-process with `oneshot`.
//! No port is opened, so the tests cannot clash with a running server.

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use notes_api::domain::{Note, NoteText};
use notes_api::http::router;
use notes_api::storage::{MemoryStore, NoteStore, SqliteStore, StoreError};
use serde_json::{Value, json};
use tower::ServiceExt; // for `oneshot`

/// Sends one request to a copy of the router and returns the status and the JSON body.
async fn call(app: &Router, method: &str, uri: &str, body: Option<Value>) -> (StatusCode, Value) {
    let mut request = Request::builder().method(method).uri(uri);
    let body = match body {
        Some(json) => {
            request = request.header("content-type", "application/json");
            Body::from(json.to_string())
        }
        None => Body::empty(),
    };
    let response = app
        .clone()
        .oneshot(request.body(body).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = match serde_json::from_slice(&bytes) {
        Ok(json) => json,
        Err(_) if bytes.is_empty() => Value::Null,
        Err(_) => Value::String(String::from_utf8_lossy(&bytes).into_owned()),
    };
    (status, body)
}

/// The behaviour every store must give the API. Run once per implementation.
async fn crud_roundtrip(app: Router) {
    let (status, body) = call(
        &app,
        "POST",
        "/notes",
        Some(json!({"text": "  learn axum "})),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body, json!({"id": 1, "text": "learn axum"}));

    call(&app, "POST", "/notes", Some(json!({"text": "ship it"}))).await;
    let (status, body) = call(&app, "GET", "/notes", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body,
        json!([{"id": 1, "text": "learn axum"}, {"id": 2, "text": "ship it"}])
    );

    let (status, body) = call(&app, "GET", "/notes/2", None).await;
    assert_eq!(
        (status, body),
        (StatusCode::OK, json!({"id": 2, "text": "ship it"}))
    );

    let (status, _) = call(&app, "DELETE", "/notes/2", None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    let (status, _) = call(&app, "GET", "/notes/2", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    // An id is not handed out again after its note is deleted.
    let (_, body) = call(&app, "POST", "/notes", Some(json!({"text": "third"}))).await;
    assert_eq!(body["id"].as_i64(), Some(3));
}

#[tokio::test]
async fn memory_store_roundtrip() {
    crud_roundtrip(router(MemoryStore::new())).await;
}

#[tokio::test]
async fn sqlite_store_roundtrip() {
    let store = SqliteStore::connect("sqlite::memory:").await.unwrap();
    crud_roundtrip(router(store)).await;
}

#[tokio::test]
async fn health_answers_ok() {
    let (status, body) = call(&router(MemoryStore::new()), "GET", "/health", None).await;
    assert_eq!((status, body), (StatusCode::OK, Value::String("ok".into())));
}

#[tokio::test]
async fn invalid_text_is_422_with_a_reason() {
    let app = router(MemoryStore::new());
    let (status, body) = call(&app, "POST", "/notes", Some(json!({"text": "   "}))).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body, json!({"error": "text is empty"}));

    let (status, body) = call(
        &app,
        "POST",
        "/notes",
        Some(json!({"text": "x".repeat(281)})),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body, json!({"error": "text is longer than 280 characters"}));

    let (_, notes) = call(&app, "GET", "/notes", None).await;
    assert_eq!(notes, json!([]), "a refused note must not be stored");
}

#[tokio::test]
async fn unknown_and_malformed_ids() {
    let app = router(MemoryStore::new());
    let (status, body) = call(&app, "GET", "/notes/99", None).await;
    assert_eq!(
        (status, body),
        (
            StatusCode::NOT_FOUND,
            json!({"error": "no note with id 99"})
        )
    );
    let (status, _) = call(&app, "DELETE", "/notes/99", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _) = call(&app, "GET", "/notes/abc", None).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

/// A store that always fails: only possible to write because the API takes a trait.
struct BrokenStore;

impl NoteStore for BrokenStore {
    async fn list(&self) -> Result<Vec<Note>, StoreError> {
        Err(StoreError::new("disk on fire"))
    }
    async fn get(&self, _id: i64) -> Result<Option<Note>, StoreError> {
        Err(StoreError::new("disk on fire"))
    }
    async fn insert(&self, _text: &NoteText) -> Result<Note, StoreError> {
        Err(StoreError::new("disk on fire"))
    }
    async fn delete(&self, _id: i64) -> Result<bool, StoreError> {
        Err(StoreError::new("disk on fire"))
    }
}

#[tokio::test]
async fn storage_failure_is_500_and_does_not_leak_details() {
    let (status, body) = call(&router(BrokenStore), "GET", "/notes", None).await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body, json!({"error": "internal error"}));
}
