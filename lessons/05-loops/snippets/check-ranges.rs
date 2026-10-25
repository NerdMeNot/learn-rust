fn v<I: Iterator<Item = i32>>(it: I) -> Vec<i32> {
    it.collect()
}

fn main() {
    assert_eq!(v(0..4), vec![0, 1, 2, 3]);
    assert_eq!(v(0..=4), vec![0, 1, 2, 3, 4]);
    assert_eq!(v((0..4).rev()), vec![3, 2, 1, 0]);
    assert_eq!(v((0..=4).rev()), vec![4, 3, 2, 1, 0]);
    assert_eq!(v((0..10).step_by(3)), vec![0, 3, 6, 9]);
    assert_eq!(v((0..=9).rev().step_by(3)), vec![9, 6, 3, 0]);
    assert_eq!(v((0..10).step_by(3).rev()), vec![9, 6, 3, 0]);
    assert_eq!(v((1..=9).rev().step_by(3)), vec![9, 6, 3]);
    assert_eq!(v((1..10).rev().step_by(3)), vec![9, 6, 3]);
    assert_eq!(v((1..10).step_by(3).rev()), vec![7, 4, 1]);
    assert_eq!(v(5..5), Vec::<i32>::new());
    assert_eq!(v(4..0), Vec::<i32>::new());
    assert_eq!(('a'..='e').collect::<String>(), "abcde");
    println!("ok");
}
