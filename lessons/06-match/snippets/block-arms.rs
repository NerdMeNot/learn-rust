fn main() {
    for seconds in [45, 150, 7200] {
        let text = match seconds {
            0..=59 => format!("{seconds} s"),
            60..=3599 => {
                let minutes = seconds / 60;
                let rest = seconds % 60;
                format!("{minutes} min {rest} s")
            }
            _ => {
                let hours = seconds / 3600;
                format!("{hours} h")
            }
        };
        println!("{text}");
    }
}
