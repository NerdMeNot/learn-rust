fn three_clause() {
    for i := 0; i < 3; i++ {
        println!("{i}");
    }
}

fn while_style(mut n: i32) {
    for n > 0 {
        n -= 1;
    }
}

fn range_style(xs: Vec<i32>) {
    for _, x := range xs {
        println!("{x}");
    }
}

fn main() {}
