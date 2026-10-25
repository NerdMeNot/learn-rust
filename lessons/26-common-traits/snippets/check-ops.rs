// Backs the table of operators and their traits. Not shown on the page.
use std::ops::{
    Add, AddAssign, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Rem, Shl, Sub,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct N(i32);

impl Add for N {
    type Output = N;
    fn add(self, o: N) -> N {
        N(self.0 + o.0)
    }
}
impl Sub for N {
    type Output = N;
    fn sub(self, o: N) -> N {
        N(self.0 - o.0)
    }
}
impl Mul for N {
    type Output = N;
    fn mul(self, o: N) -> N {
        N(self.0 * o.0)
    }
}
impl Div for N {
    type Output = N;
    fn div(self, o: N) -> N {
        N(self.0 / o.0)
    }
}
impl Rem for N {
    type Output = N;
    fn rem(self, o: N) -> N {
        N(self.0 % o.0)
    }
}
impl Neg for N {
    type Output = N;
    fn neg(self) -> N {
        N(-self.0)
    }
}
impl Not for N {
    type Output = N;
    fn not(self) -> N {
        N(!self.0)
    }
}
impl BitAnd for N {
    type Output = N;
    fn bitand(self, o: N) -> N {
        N(self.0 & o.0)
    }
}
impl BitOr for N {
    type Output = N;
    fn bitor(self, o: N) -> N {
        N(self.0 | o.0)
    }
}
impl BitXor for N {
    type Output = N;
    fn bitxor(self, o: N) -> N {
        N(self.0 ^ o.0)
    }
}
impl Shl<u32> for N {
    type Output = N;
    fn shl(self, k: u32) -> N {
        N(self.0 << k)
    }
}
impl AddAssign for N {
    fn add_assign(&mut self, o: N) {
        self.0 += o.0;
    }
}

struct Row([i32; 3]);
impl Index<usize> for Row {
    type Output = i32;
    fn index(&self, i: usize) -> &i32 {
        &self.0[i]
    }
}
impl IndexMut<usize> for Row {
    fn index_mut(&mut self, i: usize) -> &mut i32 {
        &mut self.0[i]
    }
}

// A type that is not Copy: only the arithmetic operators move it.
#[derive(Debug, PartialEq, PartialOrd)]
struct S(String);
impl Index<usize> for S {
    type Output = u8;
    fn index(&self, i: usize) -> &u8 {
        &self.0.as_bytes()[i]
    }
}
impl AddAssign<&str> for S {
    fn add_assign(&mut self, o: &str) {
        self.0.push_str(o);
    }
}
impl Add for S {
    type Output = S;
    fn add(self, o: S) -> S {
        S(self.0 + &o.0)
    }
}
// A number on the left needs its own impl: there is no reflected __radd__.
impl Mul<N> for f64 {
    type Output = N;
    fn mul(self, o: N) -> N {
        N((self * o.0 as f64) as i32)
    }
}

fn main() {
    let mut s1 = S(String::from("a"));
    let s2 = S(String::from("b"));
    // ==, < and [i] borrow, += takes &mut self: nothing moves, both stay usable
    assert!(s1 != s2 && s1 < s2 && s1[0] == b'a');
    s1 += "z";
    assert_eq!(s1, S(String::from("az")));
    assert_eq!(2.5 * N(4), N(10));
    let s3 = s1 + s2; // + takes both by value
    assert_eq!(s3, S(String::from("azb")));
    let (a, b) = (N(6), N(4));
    assert_eq!(a + b, N(10));
    assert_eq!(a - b, N(2));
    assert_eq!(a * b, N(24));
    assert_eq!(a / b, N(1));
    assert_eq!(a % b, N(2));
    assert_eq!(-a, N(-6));
    assert_eq!(!N(0), N(-1));
    assert_eq!(a & b, N(4));
    assert_eq!(a | b, N(6));
    assert_eq!(a ^ b, N(2));
    assert_eq!(b << 2, N(16));
    let mut c = a;
    c += b;
    assert_eq!(c, N(10));
    assert!(a > b && a != b);
    let mut r = Row([1, 2, 3]);
    assert_eq!(r[1], 2);
    r[1] = 20; // writing needs IndexMut
    assert_eq!(r[1], 20);
    // String + &str: the left side is taken by value
    let s = String::from("ab") + "cd";
    assert_eq!(s, "abcd");
    println!("all checks passed");
}
