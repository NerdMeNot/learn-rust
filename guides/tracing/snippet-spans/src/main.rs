use std::fmt;

use tracing::{info, info_span, instrument};
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Debug)]
struct Note {
    id: u32,
    title: &'static str,
}

struct Email(&'static str);

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Every argument becomes a field on a span that lasts for the call.
#[instrument]
fn find_note(id: u32, owner: &str) -> Option<Note> {
    info!("looking up");
    if id == 1 { Some(Note { id, title: "milk" }) } else { None }
}

fn main() {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        // also log a line each time a span is entered and exited
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
        .init();

    let note = Note { id: 7, title: "call Ana" };
    info!(id = note.id, title = note.title, "plain values");
    info!(?note, email = %Email("ana@example.com"), tags = ?["a", "b"], "Debug and Display");

    {
        let outer = info_span!("outer", user = "ana");
        let _outer = outer.enter();
        let inner = info_span!("inner", step = 2);
        let _inner = inner.enter();
        info!("inside both");
    }
    info!("outside");

    find_note(1, "ana");
    find_note(2, "bo");
}
