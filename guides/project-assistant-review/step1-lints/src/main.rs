use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct Stats {
    counts: HashMap<String, usize>,
    slowest_ms: u32,
}

fn count_line(line: &String, stats: &mut Stats) {
    let level = if line.contains("ERROR") {
        "ERROR"
    } else if line.contains("WARN") {
        "WARN"
    } else if line.contains("INFO") {
        "INFO"
    } else {
        return;
    };
    *stats.counts.entry(level.to_string()).or_insert(0) += 1;

    if let Some(pos) = line.find("ms=") {
        let ms: u64 = line[pos + 3..]
            .split_whitespace()
            .next()
            .unwrap()
            .parse()
            .unwrap();
        stats.slowest_ms = stats.slowest_ms.max(ms as u32);
    }
}

#[tokio::main]
async fn main() {
    let paths: Vec<String> = std::env::args().skip(1).collect();
    let stats = Arc::new(Mutex::new(Stats::default()));

    let mut handles = Vec::new();
    for path in paths.clone() {
        let stats = stats.clone();
        handles.push(tokio::spawn(async move {
            let contents = std::fs::read_to_string(&path).unwrap();
            for line in contents.lines() {
                let mut stats = stats.lock().unwrap();
                count_line(&line.to_string(), &mut stats);
            }
        }));
    }
    for handle in handles {
        let _ = handle.await;
    }

    let stats = stats.lock().unwrap();
    println!("files   {}", paths.len());
    for level in ["ERROR", "WARN", "INFO"] {
        let n = stats.counts.get(level).cloned().unwrap_or(0);
        println!("{level:<7} {n}");
    }
    println!("slowest {} ms", stats.slowest_ms);
}
