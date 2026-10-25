struct Team {
    red: u32,
    blue: u32,
}

/// Move up to `amount` points from `from` to `to`. Return how many moved:
/// fewer than `amount` when `from` doesn't have that many.
fn transfer(from: &mut u32, to: &mut u32, amount: u32) -> u32 {
    todo!("work out how many can move, then subtract from *from and add to *to")
}

/// The same, between two positions `i` and `j` of a list of scores.
/// If `i` and `j` are the same position, nothing moves: return 0.
fn transfer_in_list(scores: &mut Vec<u32>, i: usize, j: usize, amount: u32) -> u32 {
    todo!("two &mut scores[..] at once won't compile: read the number, then write both")
}

fn main() {
    let mut team = Team { red: 5, blue: 1 };
    // Two different fields of one struct: two `&mut` at once is fine.
    let moved = transfer(&mut team.red, &mut team.blue, 3);
    assert_eq!((moved, team.red, team.blue), (3, 2, 4));
    // Asking for more than red has moves only what is there.
    let moved = transfer(&mut team.red, &mut team.blue, 10);
    assert_eq!((moved, team.red, team.blue), (2, 0, 6));
    // Nothing left to move.
    let moved = transfer(&mut team.red, &mut team.blue, 1);
    assert_eq!((moved, team.red, team.blue), (0, 0, 6));

    let mut scores = vec![4, 0, 7];
    assert_eq!(transfer_in_list(&mut scores, 2, 1, 5), 5);
    assert_eq!(scores, [4, 5, 2]);
    assert_eq!(transfer_in_list(&mut scores, 2, 0, 9), 2);
    assert_eq!(scores, [6, 5, 0]);
    // The same position twice: nothing moves.
    assert_eq!(transfer_in_list(&mut scores, 1, 1, 3), 0);
    assert_eq!(scores, [6, 5, 0]);

    let mut single = vec![9];
    assert_eq!(transfer_in_list(&mut single, 0, 0, 4), 0);
    assert_eq!(single, [9]);

    println!("all checks passed");
}
