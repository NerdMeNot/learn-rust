// Backs the numbers in the prose of lesson.mdx. Every claim is an assertion.
fn main() {
    assert_eq!(format!("{:08.3}", -3.14159), "-003.142");
    assert_eq!(format!("{:+}", 5), "+5");
    assert_eq!(format!("{:+.1}", 2.0), "+2.0");
    assert_eq!(format!("{:08}", -42), "-0000042");
    assert_eq!(format!("{:#x}", 255), "0xff");
    assert_eq!(format!("{:#06x}", 255), "0x00ff");
    assert_eq!(format!("{:#010b}", 5), "0b00000101");
    assert_eq!(format!("{:x}", -1i8), "ff");
    assert_eq!(format!("{:x}", -255i32), "ffffff01");
    assert_eq!(format!("{:.6}", 1.5), "1.500000");
    assert_eq!(format!("{:e}", 1.5), "1.5e0");
    assert_eq!(format!("{:X}", 48879), "BEEF");
    assert_eq!(format!("{:e}", 1234.5), "1.2345e3");
    assert_eq!(format!("{:5}|", true), "true |");
    assert_eq!(format!("{:5}|", 1.5), "  1.5|");
    assert_eq!(format!("{:.3}", "Hello"), "Hel");
    assert_eq!(format!("{:.3}", 7), "7");
    assert_eq!(format!("{:.0}", 2.5), "2");
    assert_eq!(format!("{:.0}", 3.5), "4");
    assert_eq!(format!("{:.0}", -0.5), "-0");
    assert_eq!(format!("{}", 1e21), "1000000000000000000000");
    assert_eq!(format!("{:?}", 1e21), "1e21");
    assert_eq!(format!("{} {:?}", 1.0, 1.0), "1 1.0");
    println!("all rows hold");
}
