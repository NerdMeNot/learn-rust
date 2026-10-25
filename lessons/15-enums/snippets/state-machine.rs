enum Job {
    Waiting,
    Running { percent: u8 },
    Done { lines: u32 },
    Failed(String),
}

enum Event {
    Start,
    Progress(u8),
    Finish(u32),
    Crash(String),
}

impl Job {
    // takes the old state by value and returns the new one
    fn next(self, event: Event) -> Job {
        match (self, event) {
            (Job::Waiting, Event::Start) => Job::Running { percent: 0 },
            (Job::Running { .. }, Event::Progress(p)) => Job::Running { percent: p },
            (Job::Running { .. }, Event::Finish(lines)) => Job::Done { lines },
            (Job::Running { .. }, Event::Crash(why)) => Job::Failed(why),
            // any other pair is not a legal move: stay where we are
            (state, _) => state,
        }
    }

    fn show(&self) -> String {
        match self {
            Job::Waiting => String::from("waiting"),
            Job::Running { percent } => format!("running, {percent}%"),
            Job::Done { lines } => format!("done, {lines} lines"),
            Job::Failed(why) => format!("failed: {why}"),
        }
    }
}

fn main() {
    let events = [
        Event::Progress(50), // too early: the job has not started
        Event::Start,
        Event::Progress(50),
        Event::Finish(120),
        Event::Crash(String::from("late")), // too late: already done
    ];
    let mut job = Job::Waiting;
    println!("{}", job.show());
    for event in events {
        job = job.next(event);
        println!("{}", job.show());
    }
}
