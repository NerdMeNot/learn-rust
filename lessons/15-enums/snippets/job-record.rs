// One struct: every field exists in every state, so nonsense can be written.
struct JobRecord {
    running: bool,
    done: bool,
    percent: u8,   // only meaningful while running
    error: String, // only meaningful if the job failed
}

fn main() {
    let odd = JobRecord {
        running: true,
        done: true,
        percent: 250,
        error: String::from("boom"),
    };
    println!("running: {}, done: {}", odd.running, odd.done);
    println!("percent: {}, error: {:?}", odd.percent, odd.error);
}
