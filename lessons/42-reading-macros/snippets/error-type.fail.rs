struct Age(u32);

macro_rules! age {
    ($years:expr) => {
        Age($years)
    };
}

fn main() {
    let ada = age!(36);
    let linus = age!("twenty-eight");
    println!("{} {}", ada.0, linus.0);
}
