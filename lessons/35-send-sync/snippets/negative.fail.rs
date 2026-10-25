struct Handle(u32);

impl !Send for Handle {}

fn main() {
    let h = Handle(1);
    println!("{}", h.0);
}
