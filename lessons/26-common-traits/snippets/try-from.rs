#[derive(Debug)]
struct Percent(u8);

#[derive(Debug)]
struct OutOfRange(i32);

impl TryFrom<i32> for Percent {
    type Error = OutOfRange;

    fn try_from(n: i32) -> Result<Percent, OutOfRange> {
        if (0..=100).contains(&n) {
            Ok(Percent(n as u8))
        } else {
            Err(OutOfRange(n))
        }
    }
}

fn show(n: i32) {
    match Percent::try_from(n) {
        Ok(p) => println!("{n} -> Percent({})", p.0),
        Err(e) => println!("{n} -> rejected: {}", e.0),
    }
}

fn main() {
    println!("u8::try_from(200): {:?}", u8::try_from(200_i32));
    let too_big = u8::try_from(300_i32).map_err(|e| e.to_string());
    println!("u8::try_from(300): {too_big:?}");
    println!("300 as u8: {}", 300_i32 as u8);

    show(50);
    show(101);

    // try_into is the same call from the other side; the target type picks the impl.
    let checked: Result<Percent, _> = (-5_i32).try_into();
    println!("try_into -5: {checked:?}");
}
