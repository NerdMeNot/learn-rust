struct Guard(&'static str);

impl Guard {
    fn new(name: &'static str) -> Guard {
        println!("open {name}");
        Guard(name)
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        println!("close {}", self.0);
    }
}

fn main() {
    {
        let _a = Guard::new("a");
        let _b = Guard::new("b");
        println!("working");
    }
    let _c = Guard::new("c");
    println!("end of main");
}
