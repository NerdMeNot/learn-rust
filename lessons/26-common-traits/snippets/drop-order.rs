struct Noisy(String);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  drop {}", self.0);
    }
}

fn noisy(name: &str) -> Noisy {
    Noisy(String::from(name))
}

// A type with its own Drop and a field that has one too.
struct Connection {
    log: Noisy,
    stats: Noisy,
}

impl Drop for Connection {
    fn drop(&mut self) {
        println!(
            "  closing the connection (still holding {} and {})",
            self.log.0, self.stats.0
        );
    }
}

fn main() {
    println!("own Drop first, then the fields:");
    {
        let _conn = Connection {
            log: noisy("log"),
            stats: noisy("stats"),
        };
        println!("  end of block");
    }

    println!("a loop drops once per pass:");
    for i in 0..2 {
        let _guard = noisy(&format!("guard {i}"));
        println!("  end of pass {i}");
    }

    println!("shadowing does not drop the old value:");
    {
        let v = noisy("old");
        let v = Noisy(format!("{} -> new", v.0));
        println!("  end of block, v is {}", v.0);
    }
}
