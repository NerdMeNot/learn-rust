use std::fs::{self, File, OpenOptions, TryLockError};

fn state(r: Result<(), TryLockError>) -> &'static str {
    match r {
        Ok(()) => "got the lock",
        Err(TryLockError::WouldBlock) => "WouldBlock",
        Err(TryLockError::Error(_)) => "I/O error",
    }
}

fn main() -> std::io::Result<()> {
    let path = std::env::temp_dir().join("lr-lock.txt");
    let first = File::create(&path)?;
    // another handle to the same file, open for writing too: the docs leave the lock
    // calls on a read-only handle unspecified
    let second = OpenOptions::new().read(true).write(true).open(&path)?;

    first.lock()?; // waits until it can have the lock all to itself
    println!(
        "second.try_lock while first holds it: {}",
        state(second.try_lock())
    );
    println!(
        "second.try_lock_shared: {}",
        state(second.try_lock_shared())
    );

    first.unlock()?;
    println!("after first.unlock: {}", state(second.try_lock()));
    second.unlock()?;

    first.lock_shared()?;
    println!("shared + shared: {}", state(second.try_lock_shared()));
    println!("shared + exclusive: {}", state(second.try_lock()));

    drop(first);
    fs::remove_file(&path)
}
