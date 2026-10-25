struct Meters(f64);
struct Rgb(u8, u8, u8);
struct Marker;

fn main() {
    let length = Meters(3.5);
    let red = Rgb(255, 0, 0);
    let _marker = Marker;

    println!("length: {}", length.0);
    println!("red: {}, {}, {}", red.0, red.1, red.2);

    let Rgb(r, g, b) = red;
    println!("r={r} g={g} b={b}");

    println!("size of Meters: {} bytes", size_of::<Meters>());
    println!("size of f64: {} bytes", size_of::<f64>());
    println!("size of Marker: {} bytes", size_of::<Marker>());
}
