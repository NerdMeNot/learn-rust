// A job cannot be cloned or copied: it can only be moved.
struct Job {
    name: String,
}

fn job(name: &str) -> Job {
    Job {
        name: name.to_string(),
    }
}

// Remove every job whose name is longer than `max` bytes and return those jobs,
// in their original order. The jobs left in the list keep their order too.
fn take_long(jobs: &mut Vec<Job>, max: usize) -> Vec<Job> {
    todo!("move the jobs out of the list, then put the short ones back")
}

fn names(jobs: &[Job]) -> Vec<&str> {
    jobs.iter().map(|j| j.name.as_str()).collect()
}

fn main() {
    let mut jobs = vec![job("a"), job("backup"), job("cd"), job("deploy")];
    let long = take_long(&mut jobs, 2);
    assert_eq!(names(&jobs), ["a", "cd"]);
    assert_eq!(names(&long), ["backup", "deploy"]);

    let mut none: Vec<Job> = Vec::new();
    assert!(take_long(&mut none, 3).is_empty());
    assert!(none.is_empty());

    let mut all_short = vec![job("x"), job("y")];
    assert!(take_long(&mut all_short, 5).is_empty());
    assert_eq!(names(&all_short), ["x", "y"]);

    let mut all_long = vec![job("alpha"), job("beta")];
    let taken = take_long(&mut all_long, 1);
    assert!(all_long.is_empty());
    assert_eq!(names(&taken), ["alpha", "beta"]);

    // the boundary is `> max` on bytes: not `> max + 1`, and not a count of chars
    let mut edge = vec![job("abc"), job("ab"), job("éé")];
    let t = take_long(&mut edge, 2);
    assert_eq!(names(&edge), ["ab"]);
    assert_eq!(names(&t), ["abc", "éé"]);
    println!("all checks passed");
}
