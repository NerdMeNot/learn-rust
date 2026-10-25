use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// A task. The heap hands out the greatest task first, so "greater" must mean "do it sooner".
#[derive(Debug, PartialEq, Eq)]
pub struct Task {
    priority: u8,
    due: u32,
    name: &'static str,
}

/// Higher `priority` is greater. For equal priorities the earlier `due` day is greater.
/// If those tie too, the smaller `name` is greater, so that `cmp` returns `Equal` only for
/// tasks that are `==`.
impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!(
            "priority, then due day, then name: for each one, which way round must the comparison go?"
        )
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The names of the tasks in the order they should be done, using a `BinaryHeap`.
pub fn run_order(tasks: Vec<Task>) -> Vec<&'static str> {
    todo!("put the tasks in a BinaryHeap, then pop them all")
}

fn main() {
    let t = |priority, due, name| Task {
        priority,
        due,
        name,
    };

    // Each comparison is one rule of the order.
    assert!(t(2, 9, "x") > t(1, 1, "y"), "higher priority wins");
    assert!(
        t(1, 3, "x") > t(1, 5, "y"),
        "same priority: earlier due wins"
    );
    assert!(
        t(1, 3, "a") > t(1, 3, "b"),
        "same priority and due: smaller name wins"
    );
    assert_eq!(t(1, 3, "a").cmp(&t(1, 3, "a")), Ordering::Equal);
    assert_ne!(
        t(1, 3, "a").cmp(&t(1, 3, "b")),
        Ordering::Equal,
        "cmp may say Equal only for tasks that are =="
    );

    let tasks = vec![
        t(1, 4, "report"),
        t(3, 7, "deploy"),
        t(3, 2, "hotfix"),
        t(1, 4, "backup"),
        t(2, 1, "review"),
        t(3, 7, "alert"),
    ];
    assert_eq!(
        run_order(tasks),
        ["hotfix", "alert", "deploy", "review", "backup", "report"]
    );
    assert!(run_order(Vec::new()).is_empty());
    println!("all checks passed");
}
