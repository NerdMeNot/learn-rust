use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Note {
    id: u32,
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let created: Note = client
        .post("http://127.0.0.1:3000/notes")
        .json(&json!({ "text": "sent from reqwest" }))
        .send()
        .await?
        .error_for_status()? // turn 4xx and 5xx into errors
        .json()
        .await?;
    println!("created note {} with text {:?}", created.id, created.text);

    let all: Vec<Note> = client
        .get("http://127.0.0.1:3000/notes")
        .send()
        .await?
        .json()
        .await?;
    println!("the server now has {} notes", all.len());
    Ok(())
}
