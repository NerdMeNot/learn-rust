enum Job {
    Waiting,
    Failed(String),
}

struct Runner {
    job: Job,
}

impl Runner {
    fn reason(&self) -> String {
        match self.job {
            Job::Failed(msg) => msg, // tries to take the String out of &self
            Job::Waiting => String::new(),
        }
    }
}

fn main() {
    let r = Runner {
        job: Job::Failed(String::from("boom")),
    };
    println!("{}", r.reason());
    let _ = Job::Waiting;
}
