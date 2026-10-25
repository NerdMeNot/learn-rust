use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

/// Replace the file at `path` with whatever `fill` writes, so that anyone reading `path`
/// sees the old file until the new one is complete. If `fill` or anything else fails, the
/// old file must be untouched and no temporary file of yours may be left behind.
/// The temporary file lives next to `path`. Create it with `create_new` and try another
/// name if the first is taken: you must never truncate or delete a file that is not yours.
fn save_with<F>(path: &Path, fill: F) -> io::Result<()>
where
    F: FnOnce(&mut File) -> io::Result<()>,
{
    let name = path.file_name().expect("a file path").to_string_lossy();
    let mut attempt = 0;
    let (mut f, tmp) = loop {
        let tmp = path.with_file_name(format!("{name}.{}.{attempt}.tmp", std::process::id()));
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&tmp)
        {
            Ok(f) => break (f, tmp),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => attempt += 1,
            Err(e) => return Err(e),
        }
    };

    let result = (|| {
        fill(&mut f)?;
        f.sync_all()?;
        fs::rename(&tmp, path)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}

/// A fresh directory for one check, removed at the end.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Scratch {
        let dir = std::env::temp_dir().join(format!("lr-save-with-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        Scratch(dir)
    }

    fn entries(&self) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(&self.0)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn main() {
    // a new file
    let s = Scratch::new("new");
    let target = s.0.join("settings.txt");
    save_with(&target, |f| f.write_all(b"fresh")).unwrap();
    assert_eq!(fs::read_to_string(&target).unwrap(), "fresh");
    assert_eq!(
        s.entries(),
        ["settings.txt"],
        "no temporary file may be left"
    );

    // replacing an old file: while `fill` runs, readers still see the old contents,
    // and the temporary file sits in the same directory (a rename cannot cross mount points)
    let s = Scratch::new("replace");
    let target = s.0.join("settings.txt");
    fs::write(&target, "old").unwrap();
    let mut reader = File::open(&target).unwrap(); // a reader that opened the file before the save
    let mut calls = 0;
    save_with(&target, |f| {
        calls += 1;
        f.write_all(b"new")?;
        assert_eq!(
            fs::read_to_string(&target).unwrap(),
            "old",
            "the old file must stay readable"
        );
        assert_eq!(
            s.entries().len(),
            2,
            "the temporary file belongs next to the target"
        );
        Ok(())
    })
    .unwrap();
    assert_eq!(calls, 1);
    let mut seen = String::new();
    reader.read_to_string(&mut seen).unwrap();
    assert_eq!(
        seen, "old",
        "rename a new file over the path; do not write into the old one"
    );
    assert_eq!(fs::read_to_string(&target).unwrap(), "new");
    assert_eq!(s.entries(), ["settings.txt"]);

    // a failure half way: the old file survives and nothing is left over
    let s = Scratch::new("failure");
    let target = s.0.join("settings.txt");
    fs::write(&target, "old").unwrap();
    let err = save_with(&target, |f| {
        f.write_all(b"half of the ne")?;
        Err(io::Error::other("disk full"))
    })
    .unwrap_err();
    assert_eq!(
        err.to_string(),
        "disk full",
        "the error from fill must come back"
    );
    assert_eq!(
        fs::read_to_string(&target).unwrap(),
        "old",
        "a failed save must not change the file"
    );
    assert_eq!(s.entries(), ["settings.txt"], "a failed save must clean up");

    // a file that is not yours sits where the obvious temporary name would go
    let s = Scratch::new("stranger");
    let target = s.0.join("settings.txt");
    let stranger = s.0.join("settings.txt.tmp");
    fs::write(&stranger, "somebody else's").unwrap();
    save_with(&target, |f| f.write_all(b"mine")).unwrap();
    assert_eq!(fs::read_to_string(&target).unwrap(), "mine");
    assert_eq!(
        fs::read_to_string(&stranger).expect("the other file must still be there"),
        "somebody else's",
        "never truncate or delete a file you did not create"
    );
    save_with(&target, |_| Err(io::Error::other("again"))).unwrap_err();
    assert_eq!(
        s.entries(),
        ["settings.txt", "settings.txt.tmp"],
        "a failed save must leave the other file alone too"
    );

    // a directory that does not exist is an error, not a panic
    let s = Scratch::new("missing");
    let err = save_with(&s.0.join("no/such/dir/x.txt"), |f| f.write_all(b"x")).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::NotFound);
    assert!(s.entries().is_empty());

    // there is no instant at which the path does not exist: a thread opens it in a loop
    // while it is saved 500 times. Deleting first and renaming after leaves a gap; the thread
    // can only fall into it by chance, so this check is likely to catch a gap, not certain
    let s = Scratch::new("never-missing");
    let target = s.0.join("settings.txt");
    fs::write(&target, "0").unwrap();
    let stop = Arc::new(AtomicBool::new(false));
    let watcher = {
        let (target, stop) = (target.clone(), Arc::clone(&stop));
        thread::spawn(move || {
            let mut missing = 0;
            while !stop.load(Ordering::Relaxed) {
                if File::open(&target).is_err() {
                    missing += 1;
                }
            }
            missing
        })
    };
    for i in 1..=500 {
        save_with(&target, |f| f.write_all(i.to_string().as_bytes())).unwrap();
    }
    stop.store(true, Ordering::Relaxed);
    assert_eq!(
        watcher.join().unwrap(),
        0,
        "the path was missing while a save was running: rename over it, do not remove it first"
    );
    assert_eq!(fs::read_to_string(&target).unwrap(), "500");

    println!("all checks passed");
}
