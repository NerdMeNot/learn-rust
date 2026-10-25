fn main() {
    let name = "Ada";
    let n = 255;
    let pi = 3.14159265;
    println!("escape: {{ and }}");
    println!("positional: {0}-{1}-{0}", "a", "b");
    println!("named: {who} is {age}", who = name, age = 36);
    println!("inline: {name} {n}");
    println!("display vs debug: {} {:?}", name, name);
    println!(
        "width: [{:8}] [{:<8}] [{:^8}] [{:>8}]",
        name, name, name, name
    );
    println!("fill: [{:*^9}] [{:->9}]", name, name);
    println!("integers: [{:5}] [{:<5}] [{:05}] [{:+}]", n, n, n, n);
    println!("floats: [{}] [{:.2}] [{:8.3}]", pi, pi, pi);
    println!("bases: {n:x} {n:X} {n:#x} {n:b} {n:#010b} {n:o}");
    let (w, p) = (7, 1);
    println!(
        "from variables: [{:>w$}] [{:.p$}] [{:>w$.p$}]",
        name, pi, pi
    );
    println!("too long: [{:3}]", "Lovelace");
    println!("truncated: [{:.3}]", "Lovelace");
    println!("one character: [{:>4}]", "é");
}
