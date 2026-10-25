// Every Rust cell of the idiom table on the page, in the order of the table.
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

enum Value {
    Int(i32),
    Text(String),
}

trait Shape {
    fn area(&self) -> f64;
}

struct Square(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        println!("15 bye");
    }
}

const LIMIT: i32 = 100;

fn main() {
    {
        let p = Box::new(7);
        println!("1 {p}");
    }
    {
        let s = Rc::new(7);
        let t = Rc::clone(&s);
        let a = Arc::new(7);
        println!("2 {t} owners={} {}", Rc::strong_count(&s), Arc::strong_count(&a));
    }
    let mut v = vec![1, 2, 3];
    {
        v.push(4);
        println!("3 {}", v.len());
    }
    {
        println!("4 {:?}", v.get(9));
    }
    {
        let mut s = String::from("hi");
        s.push_str(" there");
        println!("5 {s}");
    }
    {
        let s: &str = "hi";
        println!("6 {}", s.len());
    }
    {
        let o: Option<i32> = Some(5);
        if let Some(x) = o {
            println!("7 {x}");
        }
    }
    {
        for w in [Value::Int(5), Value::Text(String::from("five"))] {
            match w {
                Value::Int(n) => println!("8 {n}"),
                Value::Text(t) => println!("8 {t}"),
            }
        }
    }
    {
        match "abc".parse::<i32>() {
            Ok(n) => println!("9 {n}"),
            Err(e) => println!("9 {e}"),
        }
    }
    {
        let mut sum = 0;
        for x in &v {
            sum += x;
        }
        println!("10 {sum}");
    }
    {
        let evens: Vec<i32> = v.iter().filter(|x| **x % 2 == 0).map(|x| x * 10).collect();
        println!("11 {evens:?}");
    }
    {
        let mut m: HashMap<&str, i32> = HashMap::new();
        *m.entry("a").or_insert(0) += 1;
        *m.entry("a").or_insert(0) += 1;
        println!("12 {}", m["a"]);
    }
    {
        let sh: Box<dyn Shape> = Box::new(Square(3.0));
        println!("13 {}", sh.area());
    }
    println!("14 {}", largest(3, 5));
    {
        let _g = Guard;
    }
    {
        let n = Mutex::new(0);
        *n.lock().unwrap() += 1;
        println!("16 {}", n.lock().unwrap());
    }
    {
        let result = thread::spawn(|| 42).join().unwrap();
        println!("17 {result}");
    }
    {
        let x = 3.9_f64;
        let big: i64 = 3_000_000_000;
        println!("18 {} {:?}", x as i32, i32::try_from(big).is_err());
    }
    println!("19 {LIMIT}");
    {
        let x = 5;
        let p = &x as *const i32;
        println!("20 {}", unsafe { *p });
    }
}
