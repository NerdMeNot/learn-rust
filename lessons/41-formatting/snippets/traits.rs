fn main() {
    let n = 255;
    println!("Display  {{}}:   {}", n);
    println!("Debug    {{:?}}: {:?}", "hi");
    println!("LowerHex {{:x}}:  {:x}", n);
    println!("UpperHex {{:X}}:  {:X}", n);
    println!("Octal    {{:o}}:  {:o}", n);
    println!("Binary   {{:b}}:  {:b}", n);
    println!("LowerExp {{:e}}:  {:e}", 1234.5);
    println!("UpperExp {{:E}}:  {:E}", 1234.5);
    println!("Debug in hex {{:x?}}: {:x?}", vec![255, 16]);
    let address = format!("{:p}", &n);
    println!(
        "Pointer  {{:p}}:  starts with 0x? {}",
        address.starts_with("0x")
    );
}
