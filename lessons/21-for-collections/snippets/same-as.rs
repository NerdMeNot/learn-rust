use std::any::type_name_of_val;

fn main() {
    let mut v = vec![1, 2, 3];

    println!(
        "(&v).into_iter():     {}",
        type_name_of_val(&(&v).into_iter())
    );
    println!("v.iter():             {}", type_name_of_val(&v.iter()));
    println!(
        "(&mut v).into_iter(): {}",
        type_name_of_val(&(&mut v).into_iter())
    );
    println!("v.iter_mut():         {}", type_name_of_val(&v.iter_mut()));
    // clone() so that v can be used again below
    println!(
        "v.clone().into_iter(): {}",
        type_name_of_val(&v.clone().into_iter())
    );

    let a: Vec<&i32> = (&v).into_iter().collect();
    let b: Vec<&i32> = v.iter().collect();
    println!("&v and v.iter() give the same items: {}", a == b);
}
