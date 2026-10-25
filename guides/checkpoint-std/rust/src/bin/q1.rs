fn main() {
    let csv = "a,b,,c,";
    println!("{}", csv.split(',').count());
    println!("{}", csv.split_terminator(',').count());
    println!("{}", "x\n\ny\n".lines().count());
    println!("{}", "  a  b ".split(' ').count());
    println!("{}", "  a  b ".split_whitespace().count());
}
