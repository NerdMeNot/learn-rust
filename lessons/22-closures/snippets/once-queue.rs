fn main() {
    let name = String::from("ferris");
    let mut jobs: Vec<Box<dyn FnOnce() -> String>> = Vec::new();
    jobs.push(Box::new(move || name)); // hands `name` back: it can run only once
    jobs.push(Box::new(|| String::from("second job")));

    for job in jobs {
        println!("job returned: {}", job());
    }
}
