fn slow_default(who: &str) -> u32 {
    println!("  ({who} ran the default code)");
    18
}

fn main() {
    let known: Option<u32> = Some(40);
    let missing: Option<u32> = None;

    println!("unwrap_or on Some:");
    let a = known.unwrap_or(slow_default("unwrap_or"));
    println!("  result: {a}");

    println!("unwrap_or_else on Some:");
    let b = known.unwrap_or_else(|| slow_default("unwrap_or_else"));
    println!("  result: {b}");

    println!("unwrap_or_else on None:");
    let c = missing.unwrap_or_else(|| slow_default("unwrap_or_else"));
    println!("  result: {c}");

    // unwrap_or_default uses the type's own default: 0, "", false, an empty list...
    let name: Option<String> = None;
    let nums: Option<Vec<i32>> = None;
    println!("number default: {}", missing.unwrap_or_default());
    println!("text default: {:?}", name.unwrap_or_default());
    println!("list default: {:?}", nums.unwrap_or_default());
}
