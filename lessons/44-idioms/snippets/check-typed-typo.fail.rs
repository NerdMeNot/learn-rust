#[derive(Debug)]
enum Mode {
    Read,
    Write,
}

fn main() {
    let mode = Mode::Wrtie;
    println!("{mode:?} {:?} {:?}", Mode::Read, Mode::Write);
}
