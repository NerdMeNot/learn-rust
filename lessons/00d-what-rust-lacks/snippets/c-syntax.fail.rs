struct P {
    x: i32,
}

fn arrow(p: &P) -> i32 {
    p->x
}

fn cast(n: i32) -> f64 {
    (double)n
}

fn forever() {
    for (;;) {
        break;
    }
}

fn always() {
    while (1) {
        break;
    }
}

fn print_it() {
    printf("%d\n", 5);
}

fn main() {}
