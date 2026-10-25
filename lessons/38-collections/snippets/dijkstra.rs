use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// `edges[a]` lists `(b, cost)` pairs. Returns the cheapest cost from `start` to each node.
fn shortest(edges: &[Vec<(usize, u32)>], start: usize) -> Vec<Option<u32>> {
    let mut best: Vec<Option<u32>> = vec![None; edges.len()];
    let mut todo = BinaryHeap::new();
    todo.push(Reverse((0, start)));
    while let Some(Reverse((cost, node))) = todo.pop() {
        if best[node].is_some() {
            continue; // an older, dearer entry for a node whose cost is already fixed
        }
        best[node] = Some(cost);
        for &(next, step) in &edges[node] {
            if best[next].is_none() {
                todo.push(Reverse((cost + step, next)));
            }
        }
    }
    best
}

fn main() {
    //   0 --4--> 1 --1--> 3
    //   0 --1--> 2 --2--> 1      node 4 has no way in
    //   2 --7--> 3
    let edges = vec![
        vec![(1, 4), (2, 1)],
        vec![(3, 1)],
        vec![(1, 2), (3, 7)],
        vec![],
        vec![],
    ];
    println!("costs from 0: {:?}", shortest(&edges, 0));
}
