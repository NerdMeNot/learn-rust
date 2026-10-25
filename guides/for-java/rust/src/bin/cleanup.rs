struct Res(&'static str);

impl Res {
    fn open(name: &'static str) -> Res {
        println!("open {name}");
        Res(name)
    }
}

impl Drop for Res {
    fn drop(&mut self) {
        println!("close {}", self.0);
    }
}

fn main() {
    {
        let _a = Res::open("a");
        let _b = Res::open("b");
        println!("working");
    }
    let _c = Res::open("c");
    println!("done");
}
