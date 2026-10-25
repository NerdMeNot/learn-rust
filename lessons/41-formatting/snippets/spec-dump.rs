use std::fmt::{self, Alignment, Display, Formatter};

struct Spec;

impl Display for Spec {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let align = match f.align() {
            Some(Alignment::Left) => "left",
            Some(Alignment::Right) => "right",
            Some(Alignment::Center) => "center",
            None => "none",
        };
        write!(
            f,
            "width={:?} fill={:?} align={} precision={:?} plus={} alt={} zero={}",
            f.width(),
            f.fill(),
            align,
            f.precision(),
            f.sign_plus(),
            f.alternate(),
            f.sign_aware_zero_pad()
        )
    }
}

fn main() {
    println!("{}", Spec);
    println!("{:<5}", Spec);
    println!("{:*^+#012.3}", Spec);
}
