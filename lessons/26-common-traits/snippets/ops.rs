use std::ops::{Add, AddAssign, Index, Mul, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, k: f64) -> Vec2 {
        Vec2 {
            x: self.x * k,
            y: self.y * k,
        }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

struct Path(Vec<Vec2>);

impl Index<usize> for Path {
    type Output = Vec2;
    fn index(&self, i: usize) -> &Vec2 {
        &self.0[i]
    }
}

fn main() {
    let a = Vec2 { x: 1.0, y: 2.0 };
    let b = Vec2 { x: 10.0, y: 20.0 };
    println!("a + b = {:?}", a + b);
    println!("a * 3 = {:?}", a * 3.0);
    println!("-a = {:?}", -a);
    let mut c = a;
    c += b;
    println!("c = {:?}", c);
    println!("a is still usable (Copy): {:?}", a);

    let path = Path(vec![a, b, c]);
    println!("path[1] = {:?}", path[1]);
}
