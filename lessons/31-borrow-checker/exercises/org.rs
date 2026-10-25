#![forbid(unsafe_code)]

use std::sync::atomic::{AtomicUsize, Ordering};

static CLONES: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Debug, PartialEq)]
struct Id(usize);

struct Team {
    name: String,
    size: u32,
    parent: Option<Id>,
    path: String,
}

// Counts every clone of a team, so a "fix" that copies the teams shows up in the checks.
impl Clone for Team {
    fn clone(&self) -> Team {
        CLONES.fetch_add(1, Ordering::SeqCst);
        Team {
            name: self.name.clone(),
            size: self.size,
            parent: self.parent,
            path: self.path.clone(),
        }
    }
}

// All the teams live in one Vec; a team names its parent by Id, not by reference.
#[derive(Clone)]
struct Org {
    teams: Vec<Team>,
}

impl Org {
    fn add(&mut self, name: &str, size: u32, parent: Option<Id>) -> Id {
        let path = match parent {
            Some(p) => format!("{}/{}", self.teams[p.0].path, name),
            None => name.to_string(),
        };
        self.teams.push(Team {
            name: name.to_string(),
            size,
            parent,
            path,
        });
        Id(self.teams.len() - 1)
    }

    // The team's size plus the size of every team below it, at any depth.
    fn total(&self, id: Id) -> u32 {
        let mut sum = self.teams[id.0].size;
        for (i, team) in self.teams.iter().enumerate() {
            if team.parent == Some(id) {
                sum += self.total(Id(i));
            }
        }
        sum
    }

    // Set `path` to the parent's path, a slash and the team's name (a root's path is
    // its name), then do the same for every team below it.
    fn relabel(&mut self, id: Id) {
        todo!("read the parent, write the child; then the teams below")
    }

    // Move `id` (and everything below it) under `new_parent`, and relabel.
    // Err if that would make a team its own ancestor: `new_parent` is `id` or lies below it.
    fn reparent(&mut self, id: Id, new_parent: Id) -> Result<(), String> {
        let mut at = Some(new_parent);
        while let Some(p) = at {
            if p == id {
                return Err(format!("{} cannot go under itself", self.teams[id.0].name));
            }
            at = self.teams[p.0].parent;
        }
        self.teams[id.0].parent = Some(new_parent);
        self.relabel(id);
        Ok(())
    }
}

fn main() {
    let mut org = Org { teams: Vec::new() };
    let all = org.add("all", 1, None);
    let eng = org.add("eng", 10, Some(all));
    let web = org.add("web", 4, Some(eng));
    let ops = org.add("ops", 3, Some(all));
    assert_eq!(org.total(web), 4);
    assert_eq!(org.total(eng), 14);
    assert_eq!(org.total(all), 18);
    assert_eq!(org.teams[web.0].path, "all/eng/web");

    // Growing the Vec does not invalidate any Id.
    for i in 0..100 {
        org.add("temp", 0, Some(ops));
        assert_eq!(org.total(ops), 3, "temp {i} has size 0");
    }
    assert_eq!(org.teams[web.0].name, "web");

    // Moving a team moves everything below it, and their paths follow.
    let leaf = org.add("leaf", 2, Some(web));
    assert_eq!(org.reparent(web, ops), Ok(()));
    assert_eq!(org.total(eng), 10);
    assert_eq!(org.total(ops), 9);
    assert_eq!(org.total(all), 20);
    assert_eq!(org.teams[web.0].path, "all/ops/web");
    assert_eq!(org.teams[leaf.0].path, "all/ops/web/leaf");

    // `web` now has a smaller index than its parent `ops`: do not rely on the order.
    assert_eq!(org.reparent(ops, eng), Ok(()));
    assert_eq!(org.teams[ops.0].path, "all/eng/ops");
    assert_eq!(org.teams[web.0].path, "all/eng/ops/web");
    assert_eq!(org.teams[leaf.0].path, "all/eng/ops/web/leaf");

    // A team cannot go under itself or under one of its own descendants.
    assert!(org.reparent(all, web).is_err());
    assert!(org.reparent(eng, eng).is_err());
    assert!(org.reparent(eng, leaf).is_err());
    assert_eq!(org.total(all), 20, "a refused move changes nothing");
    assert_eq!(org.teams[eng.0].path, "all/eng");

    // The teams are moved, not copied.
    assert_eq!(CLONES.load(Ordering::SeqCst), 0, "do not clone the teams");

    // A copy of the whole organisation is independent of the original.
    let mut copy = org.clone();
    copy.reparent(leaf, all).unwrap();
    assert_eq!(copy.teams[leaf.0].path, "all/leaf");
    assert_eq!(org.teams[leaf.0].path, "all/eng/ops/web/leaf");
    println!("all checks passed");
}
