// Backs the Rust column of the "you write -> in Rust" tables in lesson.mdx. Never shown.
// Every other check-* file asserts the same strings in its own language.
fn main() {
    let (n, pi, name) = (42, 3.14159, "Ada");
    assert_eq!(format!("{:5}", n), "   42");
    assert_eq!(format!("{:<5}", n), "42   ");
    assert_eq!(format!("{:05}", n), "00042");
    assert_eq!(format!("{:+}", n), "+42");
    assert_eq!(format!("{:.2}", pi), "3.14");
    assert_eq!(format!("{:8.3}", pi), "   3.142");
    assert_eq!(format!("{:x}", 255), "ff");
    assert_eq!(format!("{:#x}", 255), "0xff");
    assert_eq!(format!("{:>10}", name), "       Ada");
    assert_eq!(format!("{:10}", name), "Ada       ");
    assert_eq!(format!("{:<10}", name), "Ada       ");
    assert_eq!(format!("{:.3}", "Lovelace"), "Lov");
    assert_eq!(format!("{} {}", n, name), "42 Ada");
    assert_eq!(format!("{n} {name}"), "42 Ada");
    assert_eq!(format!("{{}}"), "{}");
    println!("ok");
}
