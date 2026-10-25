fn main() {
    for secs in [30, 1800, 4000, 90000] {
        let minutes = secs / 60;
        match minutes {
            0 => println!("{secs} s: under a minute"),
            m @ 1..=59 => println!("{secs} s: {m} min"),
            m => println!("{secs} s: {} h {} min", m / 60, m % 60),
        }
    }
}
