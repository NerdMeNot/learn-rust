use std::mem::{size_of, transmute};
use std::num::NonZeroU32;

fn main() {
    println!("--- Option costs nothing extra here ---");
    println!("&u8:               {}", size_of::<&u8>());
    println!("Option<&u8>:       {}", size_of::<Option<&u8>>());
    println!("Box<i32>:          {}", size_of::<Box<i32>>());
    println!("Option<Box<i32>>:  {}", size_of::<Option<Box<i32>>>());
    println!("NonZeroU32:        {}", size_of::<NonZeroU32>());
    println!("Option<NonZeroU32>: {}", size_of::<Option<NonZeroU32>>());
    println!("bool:              {}", size_of::<bool>());
    println!("Option<bool>:      {}", size_of::<Option<bool>>());
    println!("String:            {}", size_of::<String>());
    println!("Option<String>:    {}", size_of::<Option<String>>());

    println!("--- Here it does, because every bit pattern is a real value ---");
    println!("u32:               {}", size_of::<u32>());
    println!("Option<u32>:       {}", size_of::<Option<u32>>());
    println!("u64:               {}", size_of::<u64>());
    println!("Option<u64>:       {}", size_of::<Option<u64>>());

    println!("--- What None looks like ---");
    let value = 7u8;
    let none: Option<&u8> = None;
    let some: Option<&u8> = Some(&value);
    // unsafe: reinterpret the 8 bytes of each Option as a number, only to look at them
    let none_bits: usize = unsafe { transmute(none) };
    let some_bits: usize = unsafe { transmute(some) };
    println!("None as a number:  {none_bits}");
    println!("Some as a number is not zero: {}", some_bits != 0);
}
