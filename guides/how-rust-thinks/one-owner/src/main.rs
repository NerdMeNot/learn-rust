struct Noisy(&'static str);

// Drop runs when the owner goes away. Here it just prints, so we can watch.
impl Drop for Noisy {
    fn drop(&mut self) {
        println!("cleaning up {}", self.0);
    }
}

fn main() {
    let a = Noisy("a");
    {
        let b = Noisy("b");
        println!("using {}", b.0);
    } // b's owner ends here, so b is cleaned up here

    let c = a; // a MOVES to c: c is now the one owner
    println!("using {}", c.0);
    println!("end of main");
} // c is cleaned up here. a is not cleaned up twice: it moved.
