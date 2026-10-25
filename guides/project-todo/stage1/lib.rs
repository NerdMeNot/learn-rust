//! A to-do list: the data and its rules, with no input or output.
use std::fmt;

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

/// One thing to do.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub text: String,
    pub priority: u8,
    pub done: bool,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mark = if self.done { 'x' } else { ' ' };
        write!(
            f,
            "[{mark}] {:>2}  p{}  {}",
            self.id, self.priority, self.text
        )
    }
}

/// All the tasks, and the number the next one gets.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct List {
    next_id: u32,
    tasks: Vec<Task>,
}

impl List {
    pub fn add(&mut self, text: &str, priority: u8) -> Result<&Task> {
        let text = text.trim();
        if text.is_empty() {
            bail!("a task needs some text");
        }
        if !(1..=3).contains(&priority) {
            bail!("priority must be 1, 2 or 3, not {priority}");
        }
        self.next_id += 1;
        self.tasks.push(Task {
            id: self.next_id,
            text: text.to_string(),
            priority,
            done: false,
        });
        Ok(self.tasks.last().expect("a task was just pushed"))
    }

    pub fn complete(&mut self, id: u32) -> Result<&Task> {
        let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) else {
            bail!("no task with id {id}");
        };
        task.done = true;
        Ok(task)
    }

    pub fn remove(&mut self, id: u32) -> Result<Task> {
        let Some(pos) = self.tasks.iter().position(|t| t.id == id) else {
            bail!("no task with id {id}");
        };
        Ok(self.tasks.remove(pos))
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_count_up_and_are_not_reused() {
        let mut list = List::default();
        list.add("one", 2).unwrap();
        let two = list.add("two", 2).unwrap().id;
        list.remove(two).unwrap();
        assert_eq!(list.add("three", 2).unwrap().id, 3);
    }

    #[test]
    fn completing_marks_the_task() {
        let mut list = List::default();
        let id = list.add("one", 1).unwrap().id;
        assert!(list.complete(id).unwrap().done);
        assert!(list.tasks()[0].done);
    }

    #[test]
    fn rules_are_enforced() {
        let mut list = List::default();
        assert_eq!(
            list.add("  ", 2).unwrap_err().to_string(),
            "a task needs some text"
        );
        assert_eq!(
            list.add("x", 4).unwrap_err().to_string(),
            "priority must be 1, 2 or 3, not 4"
        );
        assert_eq!(
            list.complete(7).unwrap_err().to_string(),
            "no task with id 7"
        );
        assert!(list.tasks().is_empty());
    }

    #[test]
    fn display_marks_done_tasks() {
        let mut list = List::default();
        list.add("write", 1).unwrap();
        list.complete(1).unwrap();
        assert_eq!(list.tasks()[0].to_string(), "[x]  1  p1  write");
    }
}
