struct Pool {
    jobs: Vec<u32>,
    done: Vec<u32>,
}

impl Pool {
    // Decide with `&self`: nothing changes, so any number of borrows is fine.
    fn finished(&self) -> Vec<usize> {
        let mut plan = Vec::new();
        for (i, job) in self.jobs.iter().enumerate() {
            if job % 2 == 0 {
                plan.push(i);
            }
        }
        plan
    }

    // Apply with `&mut self`: the plan holds positions, not borrows.
    fn apply(&mut self, plan: Vec<usize>) {
        for i in plan.into_iter().rev() {
            let job = self.jobs.remove(i);
            self.done.push(job);
        }
    }

    fn tick(&mut self) {
        let plan = self.finished();
        self.apply(plan);
    }
}

fn main() {
    let mut pool = Pool {
        jobs: vec![1, 2, 3, 4, 5, 6],
        done: Vec::new(),
    };
    pool.tick();
    println!("jobs: {:?}, done: {:?}", pool.jobs, pool.done);
}
