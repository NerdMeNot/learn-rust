fn main() {
    let moves = [((0, 0), (0, 5)), ((2, 2), (2, 2)), ((1, 1), (4, 5))];
    for (from, to) in moves {
        let kind = match (from, to) {
            (a, b) if a == b => "stay",
            ((x1, _), (x2, _)) if x1 == x2 => "same column",
            ((_, y1), (_, y2)) if y1 == y2 => "same row",
            _ => "diagonal-ish",
        };
        println!("{from:?} -> {to:?}: {kind}");
    }
}
