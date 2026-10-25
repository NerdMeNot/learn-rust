use std::any::type_name_of_val;

fn main() {
    let mut v = vec![10];

    for x in v.clone() {
        println!("for x in v.clone():    x is {}", type_name_of_val(&x));
    }
    for x in &v {
        println!("for x in &v:           x is {}", type_name_of_val(&x));
    }
    for x in &mut v {
        println!("for x in &mut v:       x is {}", type_name_of_val(&x));
    }
    for x in v.iter() {
        println!("for x in v.iter():     x is {}", type_name_of_val(&x));
    }
    for x in v.iter_mut() {
        println!("for x in v.iter_mut(): x is {}", type_name_of_val(&x));
    }
    for x in v.iter().copied() {
        println!("v.iter().copied():     x is {}", type_name_of_val(&x));
    }
    for x in v {
        println!("for x in v:            x is {}", type_name_of_val(&x));
    }
}
