struct Person {
    name: String,
    age: u32,
}

fn main() {
    // the pattern takes `p` apart by value
    let p = Person {
        name: String::from("Ana"),
        age: 30,
    };
    let heap_text = p.name.as_ptr();
    let Person { name, age } = p;
    println!(
        "by value: name is the same heap text: {}",
        name.as_ptr() == heap_text
    );
    println!("by value: name len {}, cap {}", name.len(), name.capacity());
    println!("by value: age is a copy, p.age is still {}", p.age);
    println!("by value: age is {age}");

    // the pattern looks at `q` through a reference
    let q = Person {
        name: String::from("Bo"),
        age: 25,
    };
    println!(
        "by reference: q.name len {}, cap {}",
        q.name.len(),
        q.name.capacity()
    );
    let Person { name, age } = &q;
    println!(
        "by reference: name points at q.name: {}",
        std::ptr::eq(name, &q.name)
    );
    println!(
        "by reference: age points at q.age: {}",
        std::ptr::eq(age, &q.age)
    );
}
