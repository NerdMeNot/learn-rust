macro_rules! show_expr {
    ($e:expr) => {
        stringify!($e)
    };
}
macro_rules! show_ident {
    ($i:ident) => {
        stringify!($i)
    };
}
macro_rules! show_ty {
    ($t:ty) => {
        stringify!($t)
    };
}
macro_rules! show_pat {
    ($p:pat) => {
        stringify!($p)
    };
}
macro_rules! show_literal {
    ($l:literal) => {
        stringify!($l)
    };
}
macro_rules! count_tt {
    ($($t:tt)*) => {
        [$(stringify!($t)),*].len()
    };
}

fn main() {
    println!("expr:    {}", show_expr!(1 + 2 * x.len()));
    println!("ident:   {}", show_ident!(total));
    println!("ty:      {}", show_ty!(Vec<&str>));
    println!("pat:     {}", show_pat!(Some(1 | 2)));
    println!("literal: {}", show_literal!(-3.5));
    println!(
        "tt count of `a + (b, c) [d] 4`: {}",
        count_tt!(a + (b, c) [d] 4)
    );
}
