use std::any::type_name_of_val; // gives the name of a value's type

fn main() {
    println!("42 = {} ({})", 42, type_name_of_val(&42));
    println!(
        "1_000_000 = {} ({})",
        1_000_000,
        type_name_of_val(&1_000_000)
    );
    println!("0xff = {} ({})", 0xff, type_name_of_val(&0xff));
    println!("0o17 = {} ({})", 0o17, type_name_of_val(&0o17));
    println!("0b1010 = {} ({})", 0b1010, type_name_of_val(&0b1010));
    println!("7u8 = {} ({})", 7u8, type_name_of_val(&7u8));
    println!("7_i64 = {} ({})", 7_i64, type_name_of_val(&7_i64));
    println!("2.5 = {} ({})", 2.5, type_name_of_val(&2.5));
    println!("2.5f32 = {} ({})", 2.5f32, type_name_of_val(&2.5f32));
    println!("1e3 = {} ({})", 1e3, type_name_of_val(&1e3));
    println!("'R' = {} ({})", 'R', type_name_of_val(&'R'));
    println!("\"R\" = {} ({})", "R", type_name_of_val(&"R"));
    println!("b'R' = {} ({})", b'R', type_name_of_val(&b'R'));
    println!("b\"Rs\" = {:?} ({})", b"Rs", type_name_of_val(&b"Rs"));
    println!("true = {} ({})", true, type_name_of_val(&true));
    println!("() = {:?} ({})", (), type_name_of_val(&()));
}
