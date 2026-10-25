use std::cell::RefCell;
use std::panic::{self, AssertUnwindSafe};

type Events = RefCell<Vec<String>>;

/// A made-up resource. Opening it records `open <name>`; closing records `close <name>`.
struct Handle<'a> {
    events: &'a Events,
    name: &'static str,
}

impl<'a> Handle<'a> {
    fn open(events: &'a Events, name: &'static str) -> Handle<'a> {
        events.borrow_mut().push(format!("open {name}"));
        Handle { events, name }
    }
}

impl Drop for Handle<'_> {
    fn drop(&mut self) {
        self.events
            .borrow_mut()
            .push(format!("close {}", self.name));
    }
}

#[derive(Clone, Copy)]
enum Mode {
    Fine,
    Fail,
    Panic,
}

fn run(events: &Events, mode: Mode) -> Result<(), String> {
    let _db = Handle::open(events, "db");
    let _file = Handle::open(events, "file");
    events.borrow_mut().push("work".to_string());
    match mode {
        Mode::Fine => {}
        Mode::Fail => return Err("failed".to_string()),
        Mode::Panic => panic!("boom"),
    }
    events.borrow_mut().push("done".to_string());
    Ok(())
}

fn events_of(mode: Mode) -> (Vec<String>, String) {
    let events: Events = RefCell::new(Vec::new());
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| run(&events, mode)));
    let what = match outcome {
        Ok(Ok(())) => "ok",
        Ok(Err(_)) => "error",
        Err(_) => "panic",
    };
    (events.into_inner(), what.to_string())
}

fn main() {
    // Keep the expected "boom" quiet; show any other panic, such as a todo!().
    panic::set_hook(Box::new(|info| {
        if info.payload().downcast_ref::<&str>() != Some(&"boom") {
            eprintln!("{info}");
        }
    }));

    let (events, what) = events_of(Mode::Fine);
    assert_eq!(what, "ok");
    assert_eq!(
        events,
        [
            "open db",
            "open file",
            "work",
            "done",
            "close file",
            "close db"
        ]
    );

    let (events, what) = events_of(Mode::Fail);
    assert_eq!(what, "error");
    assert_eq!(
        events,
        ["open db", "open file", "work", "close file", "close db"]
    );

    let (events, what) = events_of(Mode::Panic);
    assert_eq!(what, "panic");
    assert_eq!(
        events,
        ["open db", "open file", "work", "close file", "close db"]
    );
    println!("all checks passed");
}
