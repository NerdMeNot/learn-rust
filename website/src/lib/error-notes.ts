// Plain-language notes for the compiler errors this site shows. Written by hand; everything
// else on the error index (the real message, the pages that show it) is generated from the
// extract data, so it cannot drift. An error code that appears on the site and is missing
// here still gets listed, with a reminder that it needs a note.
//
// `means` says what the compiler is telling you. `usually` is the fix people reach for.
export const ERROR_NOTES: Record<string, { means: string; usually: string }> = {
  E0004: {
    means: 'A `match` does not cover every possible value, so the compiler cannot promise one of the arms will run.',
    usually: 'Add the missing arms, or a final `_ =>` arm for "everything else". Naming every case is safer: when you add a case later, the compiler tells you where to handle it.',
  },
  E0005: {
    means: 'A `let` pattern can fail for some values (for example `let Some(x) = opt;`), and a plain `let` has nowhere to go when it does.',
    usually: 'Use `let ... else { return/continue/panic }` to say what happens on failure, or `if let` / `match` to handle both cases.',
  },
  E0061: {
    means: 'You called a function with the wrong number of arguments. Rust has no default or optional arguments and no overloading, so the count always has to match the definition.',
    usually: 'Pass every argument, or change the function. For "optional" values use `Option<T>` parameters, a small config struct, or a builder.',
  },
  E0063: {
    means: 'You built a struct and left out a field. Every field needs a value.',
    usually: 'Add the field. If the type has a sensible default, `..Default::default()` fills the rest.',
  },
  E0072: {
    means: 'A type contains itself with no pointer in between, so its size would be infinite. This happens with recursive enums and structs such as an expression tree.',
    usually: 'Put the recursive part behind a pointer: `Box<Expr>` instead of `Expr`. The `Box` lives on the heap and has a fixed size.',
  },
  E0106: {
    means: 'A function returns a reference, and the compiler cannot tell which argument it borrows from.',
    usually: 'Add a lifetime name so the signature says which argument the result borrows from, or return an owned value (`String`, `Vec`) instead.',
  },
  E0027: {
    means: 'A struct pattern has to account for every field. This one leaves some out without saying so.',
    usually: 'Name the missing fields, or end the pattern with `..` to say you do not care about the rest: `Point { x, .. }`.',
  },
  E0034: {
    means: 'Two traits in scope both give this type a method with the same name, so the call is ambiguous. Having two such traits is fine; only calling the bare name fails.',
    usually: 'Say which one you mean: `TraitA::name(&value)` or `<Type as TraitA>::name(&value)`.',
  },
  E0038: {
    means: 'You used `dyn Trait`, but the trait is not dyn compatible: a method returns `Self`, takes a generic parameter, or the trait requires `Sized` or `Clone`. A trait object cannot call those through a vtable.',
    usually: 'Take the offending method off the trait, add `where Self: Sized` to it, or use generics (`T: Trait`) instead of `dyn Trait`.',
  },
  E0046: {
    means: 'An `impl Trait for Type` block is missing a method (or associated type or constant) that the trait requires and gives no default for.',
    usually: 'Add the missing item the message names, or give it a default in the trait.',
  },
  E0107: {
    means: 'You gave the wrong number of generic arguments: too many, too few, or a turbofish on something that has none (for example on a function that takes `impl Trait`).',
    usually: 'Remove the extra arguments, or change the signature to a named type parameter (`fn f<T: Trait>(x: T)`) if callers need to write the type.',
  },
  E0117: {
    means: 'The orphan rule: you can implement a trait for a type only if the trait or the type is defined in your own crate. Here both come from elsewhere.',
    usually: 'Wrap the foreign type in your own struct (a newtype) and implement the trait for that, or define your own trait.',
  },
  E0119: {
    means: 'Two `impl` blocks could both apply to the same type, and Rust has no specialization to pick one. A blanket impl and a specific impl for the same trait overlap in this way.',
    usually: 'Remove one impl, narrow the blanket impl with a bound that excludes the specific case, or use a default method in the trait for the fallback.',
  },
  E0184: {
    means: 'A type that implements `Drop` cannot also be `Copy`. A bit-for-bit copy would run the destructor twice.',
    usually: 'Drop the `Copy` derive and use `Clone` where you need a copy, or remove the `Drop` impl.',
  },
  E0408: {
    means: 'In an or-pattern (`A(x) | B(y)`), every alternative has to bind the same names. One side binds a name the other does not.',
    usually: 'Bind the same name in every alternative, or split the alternatives into separate arms.',
  },
  E0527: {
    means: 'A slice pattern for an array has a different number of elements than the array. For arrays the length is part of the type, so the compiler checks it.',
    usually: 'Match the length exactly, or add `..` to cover the rest: `[first, ..]`. Use a slice (`&v[..]`) to match any length.',
  },
  E0530: {
    means: 'A name in a pattern is a new variable, but this name is already a `static` (or a constant-like item) and a binding cannot shadow it.',
    usually: 'Pick a different variable name, or write the path to the item if you meant to compare with it.',
  },
  E0659: {
    means: 'A name is ambiguous because two `use` globs (`use a::*; use b::*;`) both bring in an item with that name.',
    usually: 'Import the one you want explicitly (`use a::describe;`), which takes priority over globs.',
  },
  E0782: {
    means: 'A bare trait name (`Speaker`) was used where a type is needed. Since the 2021 edition a trait is not a type, so `&Speaker`, `Vec<Speaker>` and `-> Speaker` are errors.',
    usually: 'Use `&dyn Speaker` or `Box<dyn Speaker>` for a trait object, `impl Speaker` or a generic `T: Speaker` for static dispatch.',
  },
  E0500: {
    means: 'A closure needs to borrow something mutably (here all of `self`) while another borrow of it, such as an iterator over one of its fields, is still in use.',
    usually: 'Borrow only the fields the closure needs, copy what you need out first, collect before you mutate, or use a plain `for` loop and call the method after the borrow ends.',
  },
  E0728: {
    means: 'You used `.await` in a function that is not `async`. An `await` pauses the future it is in, so it needs an `async fn` or `async` block around it.',
    usually: 'Make the function `async` (and `.await` it where it is called), or keep it synchronous and have the runtime call it. This is the "function colour" rule.',
  },
  E0752: {
    means: '`main` cannot be `async`: the operating system calls it as an ordinary function, and something has to run the future.',
    usually: 'Keep `fn main()` ordinary and start a runtime inside it, or use a runtime\'s attribute such as `#[tokio::main]`, which writes that wrapper for you.',
  },
  E0195: {
    means: 'A method in your `impl` has lifetime parameters or bounds that do not match the one the trait declares. The two signatures must agree, including lifetimes, even when both look like "just a reference".',
    usually: 'Match the trait exactly. If the trait is generated with an attribute macro, put the same macro on your `impl`: for tonic that is `#[tonic::async_trait]` (a different `async_trait` gives the same error again).',
  },
  E0508: {
    means: 'You tried to move a value out of an array or slice by indexing it (`list[0]`). The element type is not `Copy`, and an index cannot leave a hole behind in the slice.',
    usually: 'Borrow the element (`&list[0]`), clone it (`list[0].clone()`), or, for a generic `T`, add a `Copy` or `Clone` bound.',
  },
  E0529: {
    means: 'A slice pattern like `[first, rest @ ..]` only matches an array or a slice, and the value you matched is something else, such as a `Vec`.',
    usually: 'Match on a slice: `match v.as_slice() { ... }` or `match &v[..] { ... }`, as the compiler\'s "consider slicing here" suggests.',
  },
  E0434: {
    means: 'A function declared with `fn` inside another function cannot use the outer function\'s local variables. Only a closure (`|x| ...`) can capture them.',
    usually: 'Pass the value in as a parameter, or write the inner function as a closure.',
  },
  E0015: {
    means: 'A `const` or `static` is worked out at compile time, and you called a function that can only run at run time (such as `String::from`). Only `const fn` functions can be called there.',
    usually: 'Use a type and value that exist at compile time (a `&str` or a number), or build the value at run time inside `main` with `let`.',
  },
  E0080: {
    means: 'The compiler runs a `const` or `static` initialiser while it compiles, and that evaluation failed: it hit undefined behaviour, overflowed, indexed out of bounds or otherwise panicked.',
    usually: 'Read the first line: it says what the evaluation did wrong. Fix the constant so the computation is valid, or make it a run-time value in `main` if it cannot be computed up front.',
  },
  E0392: {
    means: 'A generic parameter (a type or a lifetime) is declared on a struct or enum but never used in any field, so the compiler cannot tell what it is for.',
    usually: 'Use it in a field, or, if it only marks a type for the compiler, store a `PhantomData<T>` field.',
  },
  E0432: {
    means: 'A `use` names something that does not exist at that path: a misspelt item, a crate that is not a dependency, or a module that is not declared.',
    usually: 'Check the spelling and the path, add the crate to `[dependencies]`, and make sure the module is declared with `mod` where you expect it.',
  },
  E0463: {
    means: 'The compiler could not find a crate you asked for (`extern crate` or `use` of a crate that is not a dependency, or one that is not built for this target).',
    usually: 'Add it to `[dependencies]` in `Cargo.toml` and build with `cargo`, or check the target: some standard-library pieces do not exist for every target.',
  },
  E0554: {
    means: '`#![feature(...)]` turns on an unstable compiler feature, and unstable features exist only on the nightly release channel.',
    usually: 'Remove the attribute and use a stable alternative. Only use nightly deliberately, with a pinned toolchain, for features that have no stable form yet.',
  },
  E0624: {
    means: 'You called a method (or associated function) that is private to its module, from outside that module.',
    usually: 'Make it `pub` (or `pub(crate)`) if it is meant to be called from there, or call a public method that does the job.',
  },
  E0133: {
    means: 'You used something that needs an `unsafe` block (a raw pointer dereference, a call to an `unsafe` function, a `static mut`) outside of one.',
    usually: 'Most programs never need it. Look for a safe alternative first; if you really need it, wrap the smallest possible piece in `unsafe { ... }` and say why it is sound.',
  },
  E0170: {
    means: 'In a `match`, a bare name like `North` is a new variable that matches anything, not the enum variant. It is named the same as a variant, which is almost always a typo. This one is a deny-by-default lint.',
    usually: 'Write the path: `Direction::North`, or `use Direction::*;` if you want bare names.',
  },
  E0204: {
    means: 'You asked for `Copy` on a type that has a field that cannot be copied bit for bit, such as a `String` or a `Vec`.',
    usually: 'Drop `Copy` and use `Clone` when you want a copy, or change the field to something that is `Copy`.',
  },
  E0268: {
    means: '`break` or `continue` was used where no loop surrounds it. A `match` or `if` is not a loop, so there is nothing to break out of (and no switch fall-through to stop).',
    usually: 'Put the `break` inside a `loop`, `while` or `for`, or use `return` to leave a function.',
  },
  E0277: {
    means: 'A type does not implement a trait that the code needs. This is the most common error in Rust and the message varies a lot: cannot multiply an `i32` by an `f64`, cannot print with `{}`, cannot use `?` here, cannot send between threads.',
    usually: 'Read the line that starts with `the trait ... is not implemented for`. Then either convert the value (`as f64`), derive or implement the trait, or change the type.',
  },
  E0282: {
    means: 'The compiler cannot work out a type from the code around it, so it asks you to say. Typical causes: `Vec::new()` or `HashMap::new()` that nothing has used yet, and `parse()` or `collect()` without a target type.',
    usually: 'Write the type where it is first created: `let mut m: HashMap<String, Vec<u32>> = HashMap::new();`, or use the turbofish: `.collect::<Vec<_>>()`.',
  },
  E0283: {
    means: 'The compiler cannot work out which type you mean, because more than one would fit. Functions like `sum()` and `collect()` can produce many types.',
    usually: 'Say which one: `let total: i32 = ...sum();` or `.collect::<Vec<_>>()`.',
  },
  E0284: {
    means: 'Like E0282, the compiler cannot decide a type, here because a method such as `parse()` can produce many types and nothing says which. The message mentions a trait (`FromStr`) because it needs a type that implements it.',
    usually: 'Name the type you want: `let n: i32 = "42".parse().unwrap();` or `"42".parse::<i32>()`.',
  },
  E0308: {
    means: 'The compiler expected one type and found another. Typical causes: two `if` branches that give different types, a function body that ends with a semicolon, or an `Option<T>` where a `T` is wanted.',
    usually: 'Make the two sides agree: convert one, or unwrap/handle the `Option`. If a function returns nothing, check for a stray `;` on its last line.',
  },
  E0317: {
    means: 'An `if` used as a value has no `else`, so when the condition is false it would produce `()` (nothing), which is not the type of the other branch.',
    usually: 'Add an `else` that gives a value of the same type, or use `if` as a statement and put the value elsewhere.',
  },
  E0368: {
    means: 'A compound assignment such as `+=` was used on a type that does not support it. A common cause is `x += 1` where `x` is a reference (`&i32`) from a loop: the loop gave you a borrowed value, not the number itself.',
    usually: 'Dereference first (`*x += 1`) when the loop gives `&mut i32`, or loop over `&mut v` instead of `&v`. If the loop gave a copy of a plain value, changing the copy never changes the list.',
  },
  E0369: {
    means: 'An operator such as `%`, `+` or `==` is not defined for the types you gave it. A common cause is a reference to a reference, which arithmetic does not see through.',
    usually: 'Take the references off first: `*n`, `**n`, or a pattern like `|&&n|`.',
  },
  E0373: {
    means: 'A closure borrows a local variable, but the closure may run after the function that owns that variable has finished. This is what happens with `thread::spawn`.',
    usually: 'Add `move` so the closure takes ownership of what it uses.',
  },
  E0381: {
    means: 'A variable was declared but might be read before it has any value, on at least one path through the code.',
    usually: 'Give it a value where it is declared, or make sure every branch assigns it before it is used.',
  },
  E0382: {
    means: 'A value was moved somewhere else (assigned, passed to a function, put into a collection), and you used the old name afterwards.',
    usually: 'Borrow instead of moving (`&value`), clone it if you really need two (`.clone()`), or restructure so the last use comes after the move.',
  },
  E0384: {
    means: 'You assigned to a variable twice, but variables are immutable unless you say otherwise.',
    usually: 'Declare it `let mut`.',
  },
  E0407: {
    means: 'You wrote a method inside an `impl Trait for Type` block that the trait does not have. Usually a typo in the method name, because there is no `override` keyword to catch it.',
    usually: 'Check the trait definition for the exact method name and signature. The `help:` line often names the closest match.',
  },
  E0422: {
    means: 'You used a name as a struct (a literal like `Foo { .. }`) but no struct, enum variant or type alias with that name is in scope. A common cause is a typo, a missing `use`, or trying to write a class or object literal from another language.',
    usually: 'Check the spelling and add the `use`. Rust builds values with a struct declared by `struct Name { ... }` and written `Name { field: value }`.',
  },
  E0423: {
    means: 'You used a name where the compiler expected something else. A common case: calling a macro like a function (`println("hi")` instead of `println!("hi")`), or writing a struct name where a value is expected.',
    usually: 'Add the `!` for macros, or check the name: the `help:` line says which one it found.',
  },
  E0424: {
    means: 'You used `self` in a function that does not take `self` as its first parameter. Methods have to say they take `self`, `&self` or `&mut self`; there is no implicit `this`.',
    usually: 'Add `&self` (or `&mut self`, or `self`) as the first parameter, or call the function as an associated function without `self`.',
  },
  E0425: {
    means: 'The compiler cannot find a name: a variable, function or constant that is not in scope. Common causes are a typo, a variable declared inside a block you have left, or a missing `use`.',
    usually: 'Check the spelling and where the name is declared. The `help:` line often suggests a similar name that does exist.',
  },
  E0428: {
    means: 'The same name is defined twice in one scope, for example two structs or two functions with the same name.',
    usually: 'Rename one. Rust does not allow function overloading, so two functions cannot share a name even with different parameters.',
  },
  E0433: {
    means: 'A name could not be found: a module, a crate or a type is used but never brought into scope.',
    usually: 'Add the `use` line, declare the module with `mod`, or add the crate to `Cargo.toml`.',
  },
  E0435: {
    means: 'An array length (or another compile-time value) was given a normal variable. It must be a constant the compiler knows before the program runs.',
    usually: 'Use a `const`, a literal, or switch to a `Vec`, whose length can be decided at run time.',
  },
  E0451: {
    means: 'You built a struct with a literal, but one of its fields is private to the module that defines it, so code outside cannot set it.',
    usually: 'Make the field `pub`, or give the module a function such as `Item::new(...)` that builds the value for outsiders.',
  },
  E0499: {
    means: 'Two mutable borrows of the same value are alive at the same time. Rust allows one writer at a time.',
    usually: 'Finish with the first borrow before taking the second, or split the data so each borrow touches a different part.',
  },
  E0502: {
    means: 'You are changing a value while something else is still reading it (or the other way round). A reader and a writer cannot overlap.',
    usually: 'End the read first (copy out what you need), change in a separate step, or use an index instead of holding a reference.',
  },
  E0503: {
    means: 'You used a value (read or copied it) while it is mutably borrowed, and the borrow is used again later.',
    usually: 'Finish with the `&mut` first, or read the value through the reference (`*r`) instead of the owner.',
  },
  E0505: {
    means: 'You moved a value (or tried to) while a reference to it is still going to be used later.',
    usually: 'Finish using the reference before the move, or borrow the value instead of moving it. Clone only if you really need two.',
  },
  E0506: {
    means: 'You assigned a new value to something that is still borrowed. A reference that is used afterwards would suddenly see a different value.',
    usually: 'Finish using the reference first, or take the new value after the borrow ends.',
  },
  E0507: {
    means: 'You tried to take a value out of something you only borrow, such as an item of a `Vec` by index. That would leave a hole behind.',
    usually: 'Clone it, borrow it, or use a method that removes it properly: `remove`, `swap_remove`, `std::mem::take`, `Option::take`.',
  },
  E0509: {
    means: 'You tried to move a field out of a type that has its own `Drop`. Its `drop` would then see a half-empty value.',
    usually: 'Clone the field, borrow it, or use `std::mem::take` to leave something valid behind.',
  },
  E0515: {
    means: 'A function tries to return a reference to something it created itself. That something is freed when the function ends, so the reference would dangle.',
    usually: 'Return the owned value (`String`, not `&str`), or take a reference as a parameter and return a part of that.',
  },
  E0571: {
    means: '`break` with a value only works in a `loop`. A `for` or `while` loop can end without ever hitting `break`, so it has no value to give.',
    usually: 'Use `loop`, or an iterator method such as `find` or `position`.',
  },
  E0573: {
    means: 'You used an enum variant where a type is expected. `Shape::Circle` is a value of the type `Shape`, not a type of its own.',
    usually: 'Use the enum as the type (`Shape`) and match on it, or put the data in a separate struct and have the variant hold that.',
  },
  E0583: {
    means: 'You declared a module with `mod name;` but there is no `name.rs` (or `name/mod.rs`) for it.',
    usually: 'Create the file in the right folder, or use an inline module `mod name { ... }`.',
  },
  E0592: {
    means: 'Two functions with the same name in the same type. Rust has no overloading, so `new` can exist only once per type.',
    usually: 'Give them different names (`new` and `with_balance`), or take one `Option` or a small config struct.',
  },
  E0594: {
    means: 'You assigned to something you are not allowed to change: a variable that is not `mut`, or data behind a shared reference or an index that does not allow it.',
    usually: 'Make the binding `mut`, or use the type\'s own method (for a `HashMap`, `entry` or `insert`).',
  },
  E0596: {
    means: 'You called something that needs to change a value (`&mut`), but the variable is not declared `mut`, or you only hold a shared reference.',
    usually: 'Declare it `let mut`, or take `&mut` in the signature.',
  },
  E0597: {
    means: 'A reference is used after the value it points at was dropped, usually at the end of an inner block.',
    usually: 'Declare the value in the outer scope so it lives longer, or return an owned value instead of a reference to it.',
  },
  E0599: {
    means: 'The method you called does not exist for that type. Often the method is there, but the trait that provides it is not imported.',
    usually: 'Check the name. If the error mentions a trait that "is implemented but not in scope", add the `use` line it suggests.',
  },
  E0600: {
    means: 'An operator was applied to a type that does not support it. For example `-x` or `!x` on an unsigned integer or a `String` (there is no negative `u32`, and `!` on a number means bitwise not only for integers).',
    usually: 'Use a type the operator supports, or convert first (`-(x as i32)`). `x--` and `--x` are not decrement in Rust: write `x -= 1`.',
  },
  E0603: {
    means: 'You used a name from another module, but that item is private. Everything in Rust is private to its module unless it is marked `pub`.',
    usually: 'Add `pub` to the item (and to the modules on the path to it) if outside code should use it, or call a public function that does the job.',
  },
  E0605: {
    means: '`as` only converts between primitive types such as integers, floats and `char`. It cannot turn a number into your own type.',
    usually: 'Write a function that matches the number and returns the enum, or implement `TryFrom<i32>`. Converting an enum into its number with `as` is allowed.',
  },
  E0606: {
    means: 'An `as` cast between these two types is not allowed. `as` converts between number types, and a few others (`u8` to `char`, a `bool` or `char` to an integer), but not every pair: for example `char` or `bool` to `f64`.',
    usually: 'Go through a type that is allowed (`c as u32 as f64`), or use `From`/`TryFrom` (`f64::from(n)`).',
  },
  E0608: {
    means: 'You used `[...]` on a type that cannot be indexed, such as a string with a number, or an iterator.',
    usually: 'Use the right method: `.chars().nth(i)` or `&s[a..b]` for strings; `.collect::<Vec<_>>()` first for iterators.',
  },
  E0609: {
    means: 'You asked for a field that the type does not have. Sometimes the value is wrapped, such as an `Option<User>` where you wrote `user.name`.',
    usually: 'Unwrap or match the wrapper first, or check the field name.',
  },
  E0614: {
    means: 'You used `*` to dereference a value that is not a reference or pointer. The compiler is telling you the value is already the thing itself.',
    usually: 'Remove the `*`. If you expected a reference, check the type: a `for x in v` loop gives owned items, while `for x in &v` gives references.',
  },
  E0615: {
    means: 'You wrote `x.name` without parentheses, but `name` is a method, not a field. Methods are called with `()` even when they take no arguments.',
    usually: 'Call it: `x.name()`. If you really want a field, check the struct definition: the name you wrote belongs to a method.',
  },
  E0616: {
    means: 'You read or wrote a field that is private to its module.',
    usually: 'Add a method that exposes what callers need (`acct.balance()`), or make the field `pub` if outside code really should touch it directly.',
  },
  E0658: {
    means: 'The code uses a feature that exists in the compiler but has not been stabilised, so it is only available on the nightly toolchain (for example `try { }` blocks).',
    usually: 'Use a stable alternative (a closure that returns a `Result`, or a helper function and `?`), or opt into nightly only if you really need the feature.',
  },
  E0716: {
    means: 'A temporary value was freed at the end of its statement while a reference to it was still being used afterwards.',
    usually: 'Give the value a name with `let` so it lives as long as the reference does, or keep the whole chain in one statement.',
  },
}
