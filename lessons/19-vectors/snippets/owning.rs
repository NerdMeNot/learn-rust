use std::mem;

fn main() {
    let mut names = vec![
        String::from("ana"),
        String::from("bo"),
        String::from("cy"),
        String::from("di"),
    ];

    let looked_at: &String = &names[0];
    println!("borrow:      {looked_at}, list still has {}", names.len());

    let copy = names[0].clone();
    println!("clone:       {copy}, list still has {}", names.len());

    let taken = mem::take(&mut names[1]);
    println!("mem::take:   {taken:?} out, {:?} left behind", names[1]);

    names.swap(0, 3);
    println!("swap(0, 3):  {names:?}");

    let removed = names.remove(0);
    println!("remove(0):   {removed}, list is now {names:?}");

    let swapped = names.swap_remove(1);
    println!("swap_remove: {swapped}, list is now {names:?}");
}
