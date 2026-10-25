#[derive(Debug, PartialEq)]
enum Door {
    Open,
    Closed,
    Locked(u32), // the code that locks it
}

enum Action {
    Open,
    Close,
    Lock(u32),   // lock with this code
    Unlock(u32), // try to unlock with this code
}

/// The rules:
///   Closed + Open        -> Open
///   Open + Close         -> Closed
///   Closed + Lock(code)  -> Locked(code)
///   Locked(code) + Unlock(same code) -> Closed
/// Anything else (an open door cannot be locked, a locked door cannot be
/// opened, a wrong code does nothing) leaves the door as it is.
fn apply(door: Door, action: Action) -> Door {
    todo!("match on the pair (door, action)")
}

fn main() {
    assert_eq!(apply(Door::Closed, Action::Open), Door::Open);
    assert_eq!(apply(Door::Open, Action::Close), Door::Closed);
    assert_eq!(apply(Door::Closed, Action::Lock(1234)), Door::Locked(1234));
    assert_eq!(
        apply(Door::Locked(1234), Action::Unlock(1234)),
        Door::Closed
    );
    // moves that are not allowed
    assert_eq!(apply(Door::Open, Action::Lock(1)), Door::Open);
    assert_eq!(apply(Door::Locked(7), Action::Open), Door::Locked(7));
    assert_eq!(apply(Door::Locked(7), Action::Unlock(8)), Door::Locked(7));
    assert_eq!(apply(Door::Closed, Action::Unlock(7)), Door::Closed);
    // a whole story
    let mut door = Door::Open;
    for action in [
        Action::Close,
        Action::Lock(42),
        Action::Open,
        Action::Unlock(42),
        Action::Open,
    ] {
        door = apply(door, action);
    }
    assert_eq!(door, Door::Open);
    println!("all checks passed");
}
