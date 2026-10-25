use rayon::prelude::*;
use tokio::sync::oneshot;

#[derive(Debug)]
struct Reading {
    sensor: String,
    value: i64,
}

fn parse_line(line: &str) -> Result<Reading, String> {
    let (sensor, value) = line.split_once(',').ok_or(format!("{line:?}: no comma"))?;
    let value = value
        .trim()
        .parse::<i64>()
        .map_err(|e| format!("{line:?}: {e}"))?;
    Ok(Reading {
        sensor: sensor.trim().to_string(),
        value,
    })
}

// CPU-bound work goes to rayon's pool; the async fn only waits on a oneshot channel.
async fn parse_all(lines: Vec<String>) -> Result<Vec<Reading>, String> {
    let (tx, rx) = oneshot::channel();
    rayon::spawn(move || {
        let result: Result<Vec<Reading>, String> =
            lines.par_iter().map(|l| parse_line(l)).collect();
        let _ = tx.send(result);
    });
    rx.await.expect("the rayon job dropped its sender")
}

#[tokio::main]
async fn main() {
    let requests: Vec<Vec<String>> = vec![
        vec!["a,1".into(), "b,2".into(), "c,3".into()],
        vec!["d,10".into(), "e,20".into()],
        vec!["f,7".into(), "g,oops".into()],
    ];

    // three "requests" handled at once, as three tasks on the Tokio runtime
    let handles: Vec<_> = requests
        .into_iter()
        .map(|lines| tokio::spawn(parse_all(lines)))
        .collect();
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.await.unwrap() {
            Ok(readings) => {
                let total: i64 = readings.iter().map(|r| r.value).sum();
                let names: Vec<&str> = readings.iter().map(|r| r.sensor.as_str()).collect();
                println!("request {i}: {} total {total}", names.join(","));
            }
            Err(e) => println!("request {i}: error: {e}"),
        }
    }

    // a blocking call (a file read) goes to Tokio's blocking pool instead
    let lines = tokio::task::spawn_blocking(|| {
        std::fs::read_to_string("Cargo.toml").map(|s| s.lines().count())
    })
    .await
    .unwrap()
    .unwrap();
    println!("Cargo.toml has {lines} lines");
}
