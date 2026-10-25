fn main() {
    let mut v = vec![10, 20, 30, 40, 50];

    v.insert(1, 15);
    println!("insert(1, 15):    {v:?}");
    let removed = v.remove(0);
    println!("remove(0):        {v:?}, returned {removed}");
    let swapped = v.swap_remove(0);
    println!("swap_remove(0):   {v:?}, returned {swapped}");

    v.extend([60, 70]);
    println!("extend([60, 70]): {v:?}");
    let mut more = vec![80, 90];
    v.append(&mut more);
    println!("append(&mut more): {v:?}, more is now {more:?}");

    v.truncate(4);
    println!("truncate(4):      {v:?}");
    v.retain(|x| *x != 30);
    println!("retain(*x != 30): {v:?}");
    v.clear();
    println!("clear():          {v:?}, capacity still {}", v.capacity());

    let mut runs = vec![1, 1, 2, 2, 2, 1, 3, 3];
    runs.dedup();
    println!("dedup():          {runs:?}");
}
