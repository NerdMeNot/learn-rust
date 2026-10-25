fn main() {
    // Fix 1: work out what to add, then add it. The borrow has ended by then.
    let mut queue = vec![1, 2];
    let extra: Vec<i32> = queue.iter().filter(|&&n| n < 20).map(|n| n * 3).collect();
    queue.extend(extra);
    println!("collect, then extend: {queue:?}");

    // Fix 2: loop by index, when the new items should be visited too.
    // queue.len() is read again on every pass, so the loop sees what was pushed.
    let mut queue = vec![1, 2];
    let mut i = 0;
    while i < queue.len() {
        if queue[i] < 20 {
            queue.push(queue[i] * 3);
        }
        i += 1;
    }
    println!("index loop: {queue:?}");

    // Changing items in place is fine: iter_mut borrows each one mutably.
    let mut prices = vec![10, 20, 30];
    for p in prices.iter_mut() {
        *p += 1;
    }
    println!("iter_mut: {prices:?}");

    // Removing as you go: retain does it in one pass.
    let mut nums = vec![1, 2, 3, 4, 5, 6];
    nums.retain(|n| n % 3 != 0);
    println!("retain: {nums:?}");
}
