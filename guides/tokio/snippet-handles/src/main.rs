use std::num::ParseIntError;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::time::sleep;

// `?` works in an async fn that returns a Result, as in any other fn.
async fn double(text: &str) -> Result<u32, ParseIntError> {
    let n: u32 = text.parse()?;
    sleep(Duration::from_millis(5)).await;
    Ok(n * 2)
}

#[tokio::main]
async fn main() {
    // A JoinHandle gives back Result<T, JoinError>. Here T is itself a Result.
    let good = tokio::spawn(double("21"));
    let bad = tokio::spawn(double("x21"));
    println!("good: {:?}", good.await);
    println!("bad:  {:?}", bad.await);

    // A panic inside a task ends that task, not the program.
    let crash = tokio::spawn(async {
        let nothing: Option<u32> = None;
        nothing.unwrap()
    });
    let error = crash.await.unwrap_err();
    println!("panicked: {}, cancelled: {}", error.is_panic(), error.is_cancelled());

    // abort() cancels a task from outside.
    let sleeper = tokio::spawn(sleep(Duration::from_secs(3600)));
    sleeper.abort();
    let error = sleeper.await.unwrap_err();
    println!("panicked: {}, cancelled: {}", error.is_panic(), error.is_cancelled());

    // Dropping the handle does not stop the task.
    let (done, finished) = oneshot::channel();
    drop(tokio::spawn(async move {
        println!("task: still running after its handle was dropped");
        done.send(()).unwrap();
    }));
    finished.await.unwrap();
}
