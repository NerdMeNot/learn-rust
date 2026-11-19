// Plain-language notes for the compiler errors this site shows. Written by hand; everything
// else on the error index (the real message, the pages that show it) is generated from the
// extract data, so it cannot drift. An error code that appears on the site and is missing
// here still gets listed, with a reminder that it needs a note.
//
// `name` is the error in a few words, with the specifics of one program taken out: it titles
// the code's own page (/errors/e0382/), and a code with a name gets that page. `means` says
// what the compiler is telling you. `usually` is the fix people reach for. `why` (Markdown,
// may have paragraphs) says why Rust has the rule and how other languages differ, and `fixes`
// lists the usual ways out; both appear only on the code's own page.
export const ERROR_NOTES: Record<string, { name?: string; means: string; usually: string; why?: string; fixes?: string[] }> = {
  E0004: {
    name: 'non-exhaustive patterns in match',
    means: 'A `match` does not cover every possible value, so the compiler cannot promise one of the arms will run.',
    usually: 'Add the missing arms, or a final `_ =>` arm for "everything else". Naming every case is safer: when you add a case later, the compiler tells you where to handle it.',
  },
  E0005: {
    name: 'refutable pattern in local binding',
    means: 'A `let` pattern can fail for some values (for example `let Some(x) = opt;`), and a plain `let` has nowhere to go when it does.',
    usually: 'Use `let ... else { return/continue/panic }` to say what happens on failure, or `if let` / `match` to handle both cases.',
  },
  E0061: {
    name: 'wrong number of function arguments',
    means: 'You called a function with the wrong number of arguments. Rust has no default or optional arguments and no overloading, so the count always has to match the definition.',
    usually: 'Pass every argument, or change the function. For "optional" values use `Option<T>` parameters, a small config struct, or a builder.',
  },
  E0063: {
    name: 'missing fields in struct initializer',
    means: 'You built a struct and left out a field. Every field needs a value.',
    usually: 'Add the field. If the type has a sensible default, `..Default::default()` fills the rest.',
  },
  E0072: {
    name: 'recursive type has infinite size',
    means: 'A type contains itself with no pointer in between, so its size would be infinite. This happens with recursive enums and structs such as an expression tree.',
    usually: 'Put the recursive part behind a pointer: `Box<Expr>` instead of `Expr`. The `Box` lives on the heap and has a fixed size.',
    why: "Rust stores a struct or enum's fields inline, with no hidden pointer: an `Expr` value holds its fields directly, so the compiler must know its size. `Add(Expr, Expr)` would hold two more `Expr`s, each of which could hold two more, and so on without end. In Java, Kotlin, C#, Python or TypeScript a field of a class type is already a reference, so a recursive class just works. In C you write the pointer yourself (`struct Node *next`), and Rust is like C here: you choose the pointer.",
    fixes: [
      "`Box<Expr>` for a tree where each node has one owner: `Add(Box<Expr>, Box<Expr>)`.",
      "`Rc<Expr>` when several parents share one subtree.",
      "A `Vec<Expr>` of children is already indirect, because its items live on the heap.",
      "For large graphs, keep the nodes in a `Vec` and refer to them by index.",
    ],
  },
  E0106: {
    name: 'missing lifetime specifier',
    means: 'A function returns a reference, and the compiler cannot tell which argument it borrows from.',
    usually: 'Add a lifetime name so the signature says which argument the result borrows from, or return an owned value (`String`, `Vec`) instead.',
  },
  E0027: {
    name: 'pattern does not mention all fields',
    means: 'A struct pattern has to account for every field. This one leaves some out without saying so.',
    usually: 'Name the missing fields, or end the pattern with `..` to say you do not care about the rest: `Point { x, .. }`.',
    why: "Struct patterns are exhaustive about fields on purpose. When someone adds a field to the struct later, every pattern that takes it apart stops compiling, which points you at each place that may need to handle the new field. Destructuring in TypeScript (`const { x } = p`) or a class pattern in Python's `match` picks the fields it names and silently ignores the rest. In Rust, `..` is how you say \"I know there are other fields, and I don't need them here\".",
    fixes: [
      "End the pattern with `..`: `let Point { x, .. } = p;`",
      "Name every field, using `_` for the ones you ignore: `let Point { x, y: _, z: _ } = p;`",
      "If you need just one field, skip the pattern and read it: `let x = p.x;`",
    ],
  },
  E0034: {
    name: 'multiple applicable items in scope',
    means: 'Two traits in scope both give this type a method with the same name, so the call is ambiguous. Having two such traits is fine; only calling the bare name fails.',
    usually: 'Say which one you mean: `TraitA::name(&value)` or `<Type as TraitA>::name(&value)`.',
    why: "A Rust type can implement any number of traits, and the traits may come from different authors who chose the same method name. Rust accepts both `impl` blocks and only asks you to choose when a call is ambiguous. Java decides earlier: a class that inherits two unrelated default methods with the same signature must override the method. C# makes default interface methods callable only through the interface type.",
    fixes: [
      "Call the trait's method by its path: `Vehicle::start(&car)`.",
      "Use the fully qualified form when the type matters too: `<Car as Vehicle>::start(&car)`.",
      "If you own the traits, rename one of the methods so the names no longer collide.",
      "Trait methods are only callable when the trait is in scope, so in another module you can `use` just the trait you want.",
    ],
  },
  E0038: {
    name: 'the trait is not dyn compatible',
    means: 'You used `dyn Trait`, but the trait is not dyn compatible: a method returns `Self`, takes a generic parameter, or the trait requires `Sized` or `Clone`. A trait object cannot call those through a vtable.',
    usually: 'Take the offending method off the trait, add `where Self: Sized` to it, or use generics (`T: Trait`) instead of `dyn Trait`.',
  },
  E0046: {
    name: 'not all trait items implemented',
    means: 'An `impl Trait for Type` block is missing a method (or associated type or constant) that the trait requires and gives no default for.',
    usually: 'Add the missing item the message names, or give it a default in the trait.',
    why: "A trait is a contract. Every method without a default body has to be written by each type that implements the trait, like an abstract method in a Java, Kotlin or C# interface. Go works differently: a type never says which interface it implements, and the check happens where a value is used as that interface. Rust checks at the `impl`, so a missing method is reported once, where it is missing. Associated types and constants with no default count too.",
    fixes: [
      "Write the missing method in the `impl` block. rust-analyzer's \"Implement missing members\" action writes the signatures for you.",
      "If most types would write the same body, give the method a default in the trait.",
      "If only some types can provide it, move the method into a separate trait.",
    ],
  },
  E0107: {
    name: 'wrong number of generic arguments',
    means: 'You gave the wrong number of generic arguments: too many, too few, or a turbofish on something that has none (for example on a function that takes `impl Trait`).',
    usually: 'Remove the extra arguments, or change the signature to a named type parameter (`fn f<T: Trait>(x: T)`) if callers need to write the type.',
    why: "Generic parameters are positional and counted, so a type or function must get exactly as many as it declares (parameters with defaults may be left out). An `impl Trait` argument is a special case: it creates a generic parameter with no name, and you cannot fill a parameter you cannot name, so turbofish (`::<i32>`) is refused for it. The same error appears for types, as in `Vec<i32, i32>` or `HashMap<String>`.",
    fixes: [
      "Remove the turbofish and let the argument decide the type: `hidden(1)`, or `hidden(1i32)` to be explicit.",
      "If callers need to name the type, declare the parameter: `fn hidden<T: Display>(x: T)`.",
      "For a type, check how many parameters it takes in its docs: `HashMap<K, V>` takes two.",
    ],
  },
  E0117: {
    name: 'only traits defined in the current crate can be implemented (orphan rule)',
    means: 'The orphan rule: you can implement a trait for a type only if the trait or the type is defined in your own crate. Here both come from elsewhere.',
    usually: 'Wrap the foreign type in your own struct (a newtype) and implement the trait for that, or define your own trait.',
    why: "This is the orphan rule. It keeps trait implementations coherent: for any trait and type there is at most one `impl` in the whole program. If your crate and another could both implement `Display` for `Vec<i32>`, a program that used both crates would have two answers. Go has a similar rule: you can only add methods to types declared in your own package. Kotlin and C# extension methods avoid the conflict differently, by resolving the call from the imports at each call site.",
    fixes: [
      "Wrap the type in your own: `struct Numbers(Vec<i32>);` and implement the trait for `Numbers`.",
      "Define your own trait, and implement it for the foreign type.",
      "For display only, a plain function `fn describe(v: &[i32]) -> String` is often enough.",
    ],
  },
  E0119: {
    name: 'conflicting implementations of trait',
    means: 'Two `impl` blocks could both apply to the same type, and Rust has no specialization to pick one. A blanket impl and a specific impl for the same trait overlap in this way.',
    usually: 'Remove one impl, narrow the blanket impl with a bound that excludes the specific case, or use a default method in the trait for the fallback.',
  },
  E0184: {
    name: 'Copy cannot be implemented for a type with a destructor',
    means: 'A type that implements `Drop` cannot also be `Copy`. A bit-for-bit copy would run the destructor twice.',
    usually: 'Drop the `Copy` derive and use `Clone` where you need a copy, or remove the `Drop` impl.',
    why: "`Copy` means a value is duplicated with a plain copy of its bytes, and no code runs. A type with `Drop` runs code when each value dies. If such a type were `Copy`, every copy would run `drop`, and a handle would be closed or freed twice. So a type can be `Copy` or have `Drop`, never both. C++ has the same split: a type with a user-written destructor is not trivially copyable.",
    fixes: [
      "Remove `Copy` and keep `Clone`, so a second value is an explicit `.clone()`.",
      "Pass the value along (a move) instead of copying it.",
      "If several owners need one handle, share it with `Rc<Handle>` or `Arc<Handle>`, and `drop` then runs once.",
      "If the type needs no cleanup, remove the `Drop` impl.",
    ],
  },
  E0408: {
    name: 'variable is not bound in all patterns',
    means: 'In an or-pattern (`A(x) | B(y)`), every alternative has to bind the same names. One side binds a name the other does not.',
    usually: 'Bind the same name in every alternative, or split the alternatives into separate arms.',
    why: "An or-pattern `A | B` leads to one arm body, and that body uses the names the pattern bound. If `a` is bound only on the left, then when the right side is the one that matched, `a` would have no value. So each alternative must bind the same names, with the same types.",
    fixes: [
      "Split the alternatives into separate arms, each with the names it binds.",
      "Bind the name on both sides: `(Some(a), _) | (_, Some(a)) => ...`.",
      "If the body does not need the value, use `_` instead of a name.",
    ],
  },
  E0527: {
    name: 'pattern has the wrong number of elements',
    means: 'A slice pattern for an array has a different number of elements than the array. For arrays the length is part of the type, so the compiler checks it.',
    usually: 'Match the length exactly, or add `..` to cover the rest: `[first, ..]`. Use a slice (`&v[..]`) to match any length.',
    why: "An array's length is part of its type (`[i32; 3]`), so an array pattern must match that exact length. A two-element pattern can never match a three-element array, and the compiler refuses it instead of failing at run time. Python's unpacking raises `ValueError` at run time for the same mistake. JavaScript's destructuring silently ignores the extra items.",
    fixes: [
      "Bind every element: `let [x, y, z] = arr;`.",
      "Use `..` for the rest: `let [x, y, ..] = arr;` or `let [first, .., last] = arr;`.",
      "For a slice of unknown length, match it, because the pattern may not fit: `if let [x, y, ..] = slice { ... }`.",
    ],
  },
  E0530: {
    name: 'match bindings cannot shadow statics',
    means: 'A name in a pattern is a new variable, but this name is already a `static` (or a constant-like item) and a binding cannot shadow it.',
    usually: 'Pick a different variable name, or write the path to the item if you meant to compare with it.',
    why: "A name in a pattern either refers to a constant or variant, which the value is compared with, or makes a new variable. A `static` cannot be compared in a pattern, because it is a place in memory rather than a value known while compiling. Quietly treating the name as a new variable would make a catch-all arm, the bug E0170 guards against. So the compiler refuses.",
    fixes: [
      "Make it a constant, `const LIMIT: u32 = 10;`. Constants can be used in patterns.",
      "Or compare in a guard: `n if n == LIMIT => ...`.",
    ],
  },
  E0659: {
    name: 'name is ambiguous',
    means: 'A name is ambiguous because two `use` globs (`use a::*; use b::*;`) both bring in an item with that name.',
    usually: 'Import the one you want explicitly (`use a::describe;`), which takes priority over globs.',
    why: "Two glob imports (`use a::*;`) may bring in the same name. Rust allows that until the name is used, and then it is ambiguous. It does not pick the last import, the way Python's `from x import *` does. An explicit import always wins over a glob import.",
    fixes: [
      "Import the one you mean by name: `use metric::describe;`.",
      "Call it by its path: `metric::describe()`.",
      "Import the modules rather than their contents: `use crate::{imperial, metric};`.",
      "Rename one: `use imperial::describe as describe_imperial;`.",
    ],
  },
  E0782: {
    name: 'expected a type, found a trait',
    means: 'A bare trait name (`Speaker`) was used where a type is needed. Since the 2021 edition a trait is not a type, so `&Speaker`, `Vec<Speaker>` and `-> Speaker` are errors.',
    usually: 'Use `&dyn Speaker` or `Box<dyn Speaker>` for a trait object, `impl Speaker` or a generic `T: Speaker` for static dispatch.',
  },
  E0500: {
    name: 'closure requires unique access but it is already borrowed',
    means: 'A closure needs to borrow something mutably (here all of `self`) while another borrow of it, such as an iterator over one of its fields, is still in use.',
    usually: 'Borrow only the fields the closure needs, copy what you need out first, collect before you mutate, or use a plain `for` loop and call the method after the borrow ends.',
    why: "`self.items.iter()` borrows `self.items` for as long as the iterator chain runs. The closure calls `self.note(...)`, which needs `&mut self`: the whole struct, including `items`. The compiler does not look inside `note` to see that it only touches `log`, because a method's signature borrows all of `self`. Borrows of separate fields are allowed only when the code names the fields directly.",
    fixes: [
      "Borrow the two fields separately: `let log = &mut self.log;` then loop over `self.items` and push to `log`.",
      "Collect what you need first (`let names: Vec<String> = ...collect();`), then call `self.note` for each.",
      "Make the helper take only the field it changes: `fn note(log: &mut Vec<String>, text: &str)`.",
    ],
  },
  E0728: {
    name: 'await is only allowed inside async functions and blocks',
    means: 'You used `.await` in a function that is not `async`. An `await` pauses the future it is in, so it needs an `async fn` or `async` block around it.',
    usually: 'Make the function `async` (and `.await` it where it is called), or keep it synchronous and have the runtime call it. This is the "function colour" rule.',
    why: "`.await` pauses the current function until the future is ready, and only an `async` function or block can pause. The compiler turns an `async fn` into a state machine that can stop and resume, and an ordinary `fn` has no such machinery. JavaScript, Python and C# have the same rule, and Kotlin's equivalent is calling a `suspend` function from a coroutine.",
    fixes: [
      "Make the caller `async` too: `async fn total() -> u32 { read_count().await + 1 }`, and await it in turn.",
      "Inside a sync function, use an `async { ... }` block and hand the future to whatever runs it.",
      "At the top of the program, a runtime runs the first future: `#[tokio::main]`, or `block_on` from the `futures` crate. The standard library has no runtime.",
    ],
  },
  E0752: {
    name: 'main function is not allowed to be async',
    means: '`main` cannot be `async`: the operating system calls it as an ordinary function, and something has to run the future.',
    usually: 'Keep `fn main()` ordinary and start a runtime inside it, or use a runtime\'s attribute such as `#[tokio::main]`, which writes that wrapper for you.',
    why: "`main` is called by the code that starts the program, and that code cannot poll a future. Rust's standard library has no async runtime. Python has `asyncio.run` and Node.js has a built-in event loop, but in Rust you choose a runtime crate, and it is the runtime that turns an async main into a real one.",
    fixes: [
      "Use Tokio's attribute: `#[tokio::main] async fn main() { ... }`. It rewrites `main` to start a runtime.",
      "Or keep `main` sync and run one future: `fn main() { futures::executor::block_on(run()) }`.",
    ],
  },
  E0195: {
    means: 'A method in your `impl` has lifetime parameters or bounds that do not match the one the trait declares. The two signatures must agree, including lifetimes, even when both look like "just a reference".',
    usually: 'Match the trait exactly. If the trait is generated with an attribute macro, put the same macro on your `impl`: for tonic that is `#[tonic::async_trait]` (a different `async_trait` gives the same error again).',
  },
  E0508: {
    name: 'cannot move out of a non-copy array or slice',
    means: 'You tried to move a value out of an array or slice by indexing it (`list[0]`). The element type is not `Copy`, and an index cannot leave a hole behind in the slice.',
    usually: 'Borrow the element (`&list[0]`), clone it (`list[0].clone()`), or, for a generic `T`, add a `Copy` or `Clone` bound.',
    why: "`list[0]` names a place inside the slice, and `let best = list[0];` would move the value out of it. That would leave a hole in a slice the function only borrowed. For a `Copy` type such as `i32` the value is copied instead, which is why the same code works with concrete numbers. A generic `T` might be a `String`, so the compiler cannot assume a copy. The E0507 after it has the same cause.",
    fixes: [
      "Return a reference instead of a value: `fn largest<T: PartialOrd>(list: &[T]) -> &T`, starting from `&list[0]`.",
      "Require a copy: add `T: Copy` (or `T: Clone` and call `.clone()`).",
      "Use the standard library: `list.iter().max_by(|a, b| a.partial_cmp(b).unwrap())`.",
    ],
  },
  E0529: {
    name: 'expected an array or slice',
    means: 'A slice pattern like `[first, rest @ ..]` only matches an array or a slice, and the value you matched is something else, such as a `Vec`.',
    usually: 'Match on a slice: `match v.as_slice() { ... }` or `match &v[..] { ... }`, as the compiler\'s "consider slicing here" suggests.',
    why: "Slice patterns such as `[]` and `[first, ..]` work on arrays and slices. A `Vec` is a struct that points to its items. It can be borrowed as a slice, but `match` does not do that for you.",
    fixes: [
      "Match on the slice: `match v.as_slice() { ... }` or `match &v[..] { ... }`.",
      "For simple checks, use the methods: `v.is_empty()`, `v.first()`, `v.split_first()`.",
    ],
  },
  E0434: {
    name: "can't capture dynamic environment in a fn item",
    means: 'A function declared with `fn` inside another function cannot use the outer function\'s local variables. Only a closure (`|x| ...`) can capture them.',
    usually: 'Pass the value in as a parameter, or write the inner function as a closure.',
    why: "A `fn` written inside another function is an ordinary function that happens to be declared there. It is not a closure, and it cannot see the outer function's local variables. Only closures capture variables. In JavaScript, Python, Kotlin and Go, a function written inside another one can use the outer variables. In Rust that takes the closure syntax, `|args| body`.",
    fixes: [
      "Use a closure: `let tax = |p: u32| p * rate / 100;`.",
      "Pass the value in as a parameter: `fn tax(p: u32, rate: u32) -> u32`.",
      "If the value never changes, make it a constant: `const RATE: u32 = 20;`. A nested `fn` can see constants.",
    ],
  },
  E0015: {
    name: 'cannot call non-const function in a constant',
    means: 'A `const` or `static` is worked out at compile time, and you called a function that can only run at run time (such as `String::from`). Only `const fn` functions can be called there.',
    usually: 'Use a type and value that exist at compile time (a `&str` or a number), or build the value at run time inside `main` with `let`.',
  },
  E0080: {
    name: 'evaluation of constant value failed',
    means: 'The compiler runs a `const` or `static` initialiser while it compiles, and that evaluation failed: it hit undefined behaviour, overflowed, indexed out of bounds or otherwise panicked.',
    usually: 'Read the first line: it says what the evaluation did wrong. Fix the constant so the computation is valid, or make it a run-time value in `main` if it cannot be computed up front.',
    why: "Rust evaluates the initialiser of a `const` or `static` while it compiles, in an interpreter that checks each step. It refuses overflow, out-of-bounds indexing, panics and undefined behaviour. Here a `bool` may only hold 0 or 1, so making one from the byte 3 is undefined behaviour. Because the value is a constant, the compiler sees it and stops. The same `transmute` at run time would compile and quietly break the program. C++ treats `constexpr` evaluation in a similar way.",
    fixes: [
      "Make a `bool` from a number with a comparison, `byte != 0`, never with `transmute`.",
      "For overflow or indexing errors, fix the computation. The first line of the error says which step failed.",
      "If the value cannot be known until the program runs, compute it in a function instead of a `const`.",
    ],
  },
  E0392: {
    name: 'type parameter is never used',
    means: 'A generic parameter (a type or a lifetime) is declared on a struct or enum but never used in any field, so the compiler cannot tell what it is for.',
    usually: 'Use it in a field, or, if it only marks a type for the compiler, store a `PhantomData<T>` field.',
    why: "The compiler works out important facts about a generic struct from its fields: whether it owns a `T` (which matters for drop checking), how lifetimes can shrink, and whether it is `Send` and `Sync`. A parameter that no field uses gives it nothing to go on, so it is refused. Java, Kotlin, C# and TypeScript accept unused type parameters, because their generics carry no such facts. The typestate pattern needs a parameter that is only a marker, and that is what `PhantomData` is for.",
    fixes: [
      "Add a zero-size marker field: `state: PhantomData<State>` (from `std::marker`). It costs no memory.",
      "If the parameter is not needed, remove it.",
      "If it was meant to be a value, such as a size, use a const generic: `struct Buf<const N: usize>`.",
    ],
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
    name: 'unsafe operation requires an unsafe block',
    means: 'You used something that needs an `unsafe` block (a raw pointer dereference, a call to an `unsafe` function, a `static mut`) outside of one.',
    usually: 'Most programs never need it. Look for a safe alternative first; if you really need it, wrap the smallest possible piece in `unsafe { ... }` and say why it is sound.',
  },
  E0170: {
    name: 'pattern binding named the same as a variant',
    means: 'In a `match`, a bare name like `North` is a new variable that matches anything, not the enum variant. It is named the same as a variant, which is almost always a typo. This one is a deny-by-default lint.',
    usually: 'Write the path: `Direction::North`, or `use Direction::*;` if you want bare names.',
    why: "In a pattern, a bare name that is not a constant or variant in scope creates a new variable that matches anything. Variants live inside their enum (`Direction::North`), so here `North` is not in scope, and `North =>` is a catch-all that binds the value to a variable called `North`. The `South` arm can never run. This bug is common enough that the compiler makes it an error. The habit comes from Java's `switch`, where enum constants are written bare, and from C and C++, whose plain enums put their names in the surrounding scope.",
    fixes: [
      "Qualify the variant: `Direction::North => true`.",
      "Bring the variants into scope: `use Direction::*;` or `use Direction::{North, South};`.",
      "Inside an `impl Direction` block, `Self::North` works too.",
    ],
  },
  E0204: {
    name: 'the trait Copy cannot be implemented for this type',
    means: 'You asked for `Copy` on a type that has a field that cannot be copied bit for bit, such as a `String` or a `Vec`.',
    usually: 'Drop `Copy` and use `Clone` when you want a copy, or change the field to something that is `Copy`.',
  },
  E0268: {
    means: '`break` or `continue` was used where no loop surrounds it. A `match` or `if` is not a loop, so there is nothing to break out of (and no switch fall-through to stop).',
    usually: 'Put the `break` inside a `loop`, `while` or `for`, or use `return` to leave a function.',
  },
  E0277: {
    name: 'the trait bound is not satisfied',
    means: 'A type does not implement a trait that the code needs. This is the most common error in Rust and the message varies a lot: cannot multiply an `i32` by an `f64`, cannot print with `{}`, cannot use `?` here, cannot send between threads.',
    usually: 'Read the line that starts with `the trait ... is not implemented for`. Then either convert the value (`as f64`), derive or implement the trait, or change the type.',
  },
  E0282: {
    name: 'type annotations needed',
    means: 'The compiler cannot work out a type from the code around it, so it asks you to say. Typical causes: `Vec::new()` or `HashMap::new()` that nothing has used yet, and `parse()` or `collect()` without a target type.',
    usually: 'Write the type where it is first created: `let mut m: HashMap<String, Vec<u32>> = HashMap::new();`, or use the turbofish: `.collect::<Vec<_>>()`.',
  },
  E0283: {
    name: 'type annotations needed: more than one type fits',
    means: 'The compiler cannot work out which type you mean, because more than one would fit. Functions like `sum()` and `collect()` can produce many types.',
    usually: 'Say which one: `let total: i32 = ...sum();` or `.collect::<Vec<_>>()`.',
  },
  E0284: {
    name: 'type annotations needed: cannot satisfy a trait projection',
    means: 'Like E0282, the compiler cannot decide a type, here because a method such as `parse()` can produce many types and nothing says which. The message mentions a trait (`FromStr`) because it needs a type that implements it.',
    usually: 'Name the type you want: `let n: i32 = "42".parse().unwrap();` or `"42".parse::<i32>()`.',
    why: "`parse` is generic over the type it returns: any type that implements `FromStr`. The compiler picks that type from how the result is used. Here the result is only printed, which works for many types, so nothing decides. The note about `<_ as FromStr>::Err` says the compiler could not even work out the error type. Python's `int(\"42\")` and Go's `strconv.Atoi` put the target type in the function's name. Rust puts it in the type.",
    fixes: [
      "Give the variable a type: `let n: i32 = \"42\".parse().unwrap();`.",
      "Use the turbofish: `\"42\".parse::<i32>()`.",
      "Pass the result straight to something whose parameter type is known, and inference fills it in.",
    ],
  },
  E0308: {
    name: 'mismatched types',
    means: 'The compiler expected one type and found another. Typical causes: two `if` branches that give different types, a function body that ends with a semicolon, or an `Option<T>` where a `T` is wanted.',
    usually: 'Make the two sides agree: convert one, or unwrap/handle the `Option`. If a function returns nothing, check for a stray `;` on its last line.',
  },
  E0317: {
    name: 'if may be missing an else clause',
    means: 'An `if` used as a value has no `else`, so when the condition is false it would produce `()` (nothing), which is not the type of the other branch.',
    usually: 'Add an `else` that gives a value of the same type, or use `if` as a statement and put the value elsewhere.',
    why: "In Rust `if` is an expression with a value. With no `else`, the false case produces `()`, the empty value, so the whole `if` has type `()` and cannot be a `&str`. There is no fallback like `null`, `undefined` or a zero value. Python refuses the same thing for its conditional expression (`\"big\" if n > 5` with no `else` is a syntax error). In Go or JavaScript you would declare the variable first and leave it empty when the condition is false.",
    fixes: [
      "Add an `else` that gives a value of the same type: `if n > 5 { \"big\" } else { \"small\" }`.",
      "If there may be no value, say so with `Option`: `let size = (n > 5).then_some(\"big\");`.",
      "Or declare first and assign in every branch: `let size; if n > 5 { size = \"big\" } else { size = \"small\" }`.",
    ],
  },
  E0368: {
    name: 'binary assignment operation cannot be applied to type',
    means: 'A compound assignment such as `+=` was used on a type that does not support it. A common cause is `x += 1` where `x` is a reference (`&i32`) from a loop: the loop gave you a borrowed value, not the number itself.',
    usually: 'Dereference first (`*x += 1`) when the loop gives `&mut i32`, or loop over `&mut v` instead of `&v`. If the loop gave a copy of a plain value, changing the copy never changes the list.',
  },
  E0369: {
    name: 'binary operation cannot be applied to type',
    means: 'An operator such as `%`, `+` or `==` is not defined for the types you gave it. A common cause is a reference to a reference, which arithmetic does not see through.',
    usually: 'Take the references off first: `*n`, `**n`, or a pattern like `|&&n|`.',
  },
  E0373: {
    name: 'closure may outlive the current function',
    means: 'A closure borrows a local variable, but the closure may run after the function that owns that variable has finished. This is what happens with `thread::spawn`.',
    usually: 'Add `move` so the closure takes ownership of what it uses.',
  },
  E0381: {
    name: 'used binding is not initialized',
    means: 'A variable was declared but might be read before it has any value, on at least one path through the code.',
    usually: 'Give it a value where it is declared, or make sure every branch assigns it before it is used.',
  },
  E0382: {
    name: 'use of moved value',
    means: 'A value was moved somewhere else (assigned, passed to a function, put into a collection), and you used the old name afterwards.',
    usually: 'Borrow instead of moving (`&value`), clone it if you really need two (`.clone()`), or restructure so the last use comes after the move.',
  },
  E0384: {
    name: 'cannot assign twice to immutable variable',
    means: 'You assigned to a variable twice, but variables are immutable unless you say otherwise.',
    usually: 'Declare it `let mut`.',
  },
  E0407: {
    name: 'method is not a member of trait',
    means: 'You wrote a method inside an `impl Trait for Type` block that the trait does not have. Usually a typo in the method name, because there is no `override` keyword to catch it.',
    usually: 'Check the trait definition for the exact method name and signature. The `help:` line often names the closest match.',
    why: "Every method in an `impl Trait for Type` block has to be declared in the trait. Here it is a typo, `perimiter`, and Rust catches it the way `@Override` in Java or `override` in C# and Kotlin catches a misspelt override. In Python, Go or TypeScript a misspelt method is just a new method, and the default is used without warning. Methods that belong to the type but not the trait go in a separate `impl Square` block.",
    fixes: [
      "Fix the spelling to match the trait. The trait definition is the source of truth.",
      "Move extra methods into a plain `impl Square { ... }` block.",
      "If every type with the trait should have the method, add it to the trait.",
    ],
  },
  E0422: {
    name: 'cannot find struct, variant or union type',
    means: 'You used a name as a struct (a literal like `Foo { .. }`) but no struct, enum variant or type alias with that name is in scope. A common cause is a typo, a missing `use`, or trying to write a class or object literal from another language.',
    usually: 'Check the spelling and add the `use`. Rust builds values with a struct declared by `struct Name { ... }` and written `Name { field: value }`.',
    why: "Rust has no `switch`. `match` does that job. The compiler read `switch { ... }` as a struct literal, because `Name { ... }` builds a struct, and there is no type called `switch`. The first errors in this output are parse errors with no code, for `defer`, tuple assignment and `goto`. The compiler reports what it can and keeps going, which is why E0422 comes last.",
    fixes: [
      "Use `match` with guards: `match x { n if n > 5 => println!(\"big\"), _ => {} }`.",
      "For a plain chain of conditions, `if` / `else if` is fine.",
      "If you did mean a struct literal, check the type's spelling and that it is in scope (`use`).",
    ],
  },
  E0423: {
    name: 'expected value, found something else',
    means: 'You used a name where the compiler expected something else. A common case: calling a macro like a function (`println("hi")` instead of `println!("hi")`), or writing a struct name where a value is expected.',
    usually: 'Add the `!` for macros, or check the name: the `help:` line says which one it found.',
    why: "Macros and functions live in separate namespaces. `println` is a macro, so `println(...)` looks for a function and finds none. Macros are marked with `!` because they are expanded while the program compiles. That is how `println!` takes any number of arguments and checks its format string, which a Rust function cannot do. The same error appears for `vec(...)` and `format(...)`, and when a struct name is used as a value (`let p = Point;` for a struct with fields).",
    fixes: [
      "Add the `!`: `println!(\"hello\")`.",
      "For a struct, build it with its fields, `Point { x: 1, y: 2 }`, or its constructor, `Point::new(1, 2)`.",
    ],
  },
  E0424: {
    means: 'You used `self` in a function that does not take `self` as its first parameter. Methods have to say they take `self`, `&self` or `&mut self`; there is no implicit `this`.',
    usually: 'Add `&self` (or `&mut self`, or `self`) as the first parameter, or call the function as an associated function without `self`.',
  },
  E0425: {
    name: 'cannot find value or function in this scope',
    means: 'The compiler cannot find a name: a variable, function or constant that is not in scope. Common causes are a typo, a variable declared inside a block you have left, or a missing `use`.',
    usually: 'Check the spelling and where the name is declared. The `help:` line often suggests a similar name that does exist.',
  },
  E0428: {
    name: 'the name is defined multiple times',
    means: 'The same name is defined twice in one scope, for example two structs or two functions with the same name.',
    usually: 'Rename one. Rust does not allow function overloading, so two functions cannot share a name even with different parameters.',
    why: "Rust has no function overloading. In a module, one name means one function. Java, C#, C++ and Kotlin pick an overload from the argument types. Rust does not, partly because overloading would make type inference much harder. Go and C have no overloading either. In Python, a second `def` with the same name silently replaces the first, and Rust refuses instead. The E0308 after it follows from the first error: the compiler kept the first `show`, which takes an `i32`.",
    fixes: [
      "Give each function its own name: `show_int`, `show_text`.",
      "Write one generic function over a trait: `fn show(x: impl Display) -> String`.",
      "Define a trait with a `show` method and implement it for each type.",
      "Take an enum, if the set of input kinds is fixed.",
    ],
  },
  E0433: {
    name: 'failed to resolve: undeclared type or module',
    means: 'A name could not be found: a module, a crate or a type is used but never brought into scope.',
    usually: 'Add the `use` line, declare the module with `mod`, or add the crate to `Cargo.toml`.',
  },
  E0435: {
    name: 'attempt to use a non-constant value in a constant',
    means: 'An array length (or another compile-time value) was given a normal variable. It must be a constant the compiler knows before the program runs.',
    usually: 'Use a `const`, a literal, or switch to a `Vec`, whose length can be decided at run time.',
  },
  E0451: {
    name: 'field of struct is private',
    means: 'You built a struct with a literal, but one of its fields is private to the module that defines it, so code outside cannot set it.',
    usually: 'Make the field `pub`, or give the module a function such as `Item::new(...)` that builds the value for outsiders.',
  },
  E0499: {
    name: 'cannot borrow as mutable more than once at a time',
    means: 'Two mutable borrows of the same value are alive at the same time. Rust allows one writer at a time.',
    usually: 'Finish with the first borrow before taking the second, or split the data so each borrow touches a different part.',
  },
  E0502: {
    name: 'cannot borrow as mutable because it is also borrowed as immutable',
    means: 'You are changing a value while something else is still reading it (or the other way round). A reader and a writer cannot overlap.',
    usually: 'End the read first (copy out what you need), change in a separate step, or use an index instead of holding a reference.',
  },
  E0503: {
    means: 'You used a value (read or copied it) while it is mutably borrowed, and the borrow is used again later.',
    usually: 'Finish with the `&mut` first, or read the value through the reference (`*r`) instead of the owner.',
  },
  E0505: {
    name: 'cannot move out because it is borrowed',
    means: 'You moved a value (or tried to) while a reference to it is still going to be used later.',
    usually: 'Finish using the reference before the move, or borrow the value instead of moving it. Clone only if you really need two.',
  },
  E0506: {
    name: 'cannot assign because it is borrowed',
    means: 'You assigned a new value to something that is still borrowed. A reference that is used afterwards would suddenly see a different value.',
    usually: 'Finish using the reference first, or take the new value after the borrow ends.',
  },
  E0507: {
    name: 'cannot move out of a borrowed value',
    means: 'You tried to take a value out of something you only borrow, such as an item of a `Vec` by index. That would leave a hole behind.',
    usually: 'Clone it, borrow it, or use a method that removes it properly: `remove`, `swap_remove`, `std::mem::take`, `Option::take`.',
  },
  E0509: {
    name: 'cannot move out of a type that implements Drop',
    means: 'You tried to move a field out of a type that has its own `Drop`. Its `drop` would then see a half-empty value.',
    usually: 'Clone the field, borrow it, or use `std::mem::take` to leave something valid behind.',
    why: "A type with `Drop` runs its `drop` on the whole value when it goes out of scope. If one field had been moved out, `drop` would read a field that is no longer there. So you cannot move a field out of a type that implements `Drop`. Without the `Drop` impl, the same partial move is allowed.",
    fixes: [
      "Borrow the field: `let user = &session.user;`.",
      "Clone it, if you need your own copy: `session.user.clone()`.",
      "Take it and leave an empty value behind: `std::mem::take(&mut session.user)` (the binding must be `mut`).",
      "Give the type a method that hands the field over, such as `fn into_user(mut self) -> String`, built on `mem::take`.",
    ],
  },
  E0515: {
    name: 'cannot return reference to local variable',
    means: 'A function tries to return a reference to something it created itself. That something is freed when the function ends, so the reference would dangle.',
    usually: 'Return the owned value (`String`, not `&str`), or take a reference as a parameter and return a part of that.',
  },
  E0571: {
    name: 'break with value from a for loop',
    means: '`break` with a value only works in a `loop`. A `for` or `while` loop can end without ever hitting `break`, so it has no value to give.',
    usually: 'Use `loop`, or an iterator method such as `find` or `position`.',
  },
  E0573: {
    name: 'expected type, found variant',
    means: 'You used an enum variant where a type is expected. `Shape::Circle` is a value of the type `Shape`, not a type of its own.',
    usually: 'Use the enum as the type (`Shape`) and match on it, or put the data in a separate struct and have the variant hold that.',
    why: "In Rust an enum's variants are not types. Only the enum is a type. `Shape::Circle` is a constructor and a pattern, and it cannot be the type of a parameter. Kotlin's sealed subclasses, Java's sealed records and TypeScript's union members are types of their own, so the habit carries over.",
    fixes: [
      "Take the enum, and `match` inside: `fn radius(s: &Shape) -> Option<f64>`.",
      "If the variant needs its own type, give it a struct: `struct Circle { r: f64 }` and `enum Shape { Circle(Circle), ... }`, then `fn radius(c: &Circle)`.",
    ],
  },
  E0583: {
    means: 'You declared a module with `mod name;` but there is no `name.rs` (or `name/mod.rs`) for it.',
    usually: 'Create the file in the right folder, or use an inline module `mod name { ... }`.',
  },
  E0592: {
    name: 'duplicate definitions with the same name',
    means: 'Two functions with the same name in the same type. Rust has no overloading, so `new` can exist only once per type.',
    usually: 'Give them different names (`new` and `with_balance`), or take one `Option` or a small config struct.',
  },
  E0594: {
    name: 'cannot assign to an immutable value',
    means: 'You assigned to something you are not allowed to change: a variable that is not `mut`, or data behind a shared reference or an index that does not allow it.',
    usually: 'Make the binding `mut`, or use the type\'s own method (for a `HashMap`, `entry` or `insert`).',
  },
  E0596: {
    name: 'cannot borrow as mutable',
    means: 'You called something that needs to change a value (`&mut`), but the variable is not declared `mut`, or you only hold a shared reference.',
    usually: 'Declare it `let mut`, or take `&mut` in the signature.',
  },
  E0597: {
    name: 'borrowed value does not live long enough',
    means: 'A reference is used after the value it points at was dropped, usually at the end of an inner block.',
    usually: 'Declare the value in the outer scope so it lives longer, or return an owned value instead of a reference to it.',
  },
  E0599: {
    name: 'no method found for this type',
    means: 'The method you called does not exist for that type. Often the method is there, but the trait that provides it is not imported.',
    usually: 'Check the name. If the error mentions a trait that "is implemented but not in scope", add the `use` line it suggests.',
  },
  E0600: {
    name: 'cannot apply unary operator',
    means: 'An operator was applied to a type that does not support it. For example `-x` or `!x` on an unsigned integer or a `String` (there is no negative `u32`, and `!` on a number means bitwise not only for integers).',
    usually: 'Use a type the operator supports, or convert first (`-(x as i32)`). `x--` and `--x` are not decrement in Rust: write `x -= 1`.',
    why: "Rust has no `++` or `--`. `--count` is read as `-(-count)`, negation twice, and an unsigned `u32` cannot be negated at all, so it fails here. On a signed `i32` it would compile and give back the same number, with only a lint warning to flag it.",
    fixes: [
      "Change the variable: `count -= 1;`.",
      "Or compute a new value: `let after = count - 1;`.",
      "If `count` may be 0, use `count.checked_sub(1)`, which returns an `Option`. A plain `- 1` panics in a debug build and wraps around in a release build.",
    ],
  },
  E0603: {
    name: 'item is private',
    means: 'You used a name from another module, but that item is private. Everything in Rust is private to its module unless it is marked `pub`.',
    usually: 'Add `pub` to the item (and to the modules on the path to it) if outside code should use it, or call a public function that does the job.',
  },
  E0605: {
    name: 'non-primitive cast',
    means: '`as` only converts between primitive types such as integers, floats and `char`. It cannot turn a number into your own type.',
    usually: 'Write a function that matches the number and returns the enum, or implement `TryFrom<i32>`. Converting an enum into its number with `as` is allowed.',
  },
  E0606: {
    name: 'invalid cast between types',
    means: 'An `as` cast between these two types is not allowed. `as` converts between number types, and a few others (`u8` to `char`, a `bool` or `char` to an integer), but not every pair: for example `char` or `bool` to `f64`.',
    usually: 'Go through a type that is allowed (`c as u32 as f64`), or use `From`/`TryFrom` (`f64::from(n)`).',
    why: "`as` allows only a fixed list of conversions between primitive types. A `char` can become an integer (its Unicode code point), but there is no meaning for a character as a floating-point number. So the compiler asks you to go through an integer, which makes the conversion explicit.",
    fixes: [
      "Cast through an integer: `'a' as u32 as f64`.",
      "Or use the lossless conversions: `f64::from(u32::from('a'))`.",
      "If you meant the digit's value, `'7'.to_digit(10)` gives `Some(7)`.",
    ],
  },
  E0608: {
    name: 'cannot index into a value of this type',
    means: 'You used `[...]` on a type that cannot be indexed, such as a string with a number, or an iterator.',
    usually: 'Use the right method: `.chars().nth(i)` or `&s[a..b]` for strings; `.collect::<Vec<_>>()` first for iterators.',
  },
  E0609: {
    name: 'no field on type',
    means: 'You asked for a field that the type does not have. Sometimes the value is wrapped, such as an `Option<User>` where you wrote `user.name`.',
    usually: 'Unwrap or match the wrapper first, or check the field name.',
  },
  E0614: {
    means: 'You used `*` to dereference a value that is not a reference or pointer. The compiler is telling you the value is already the thing itself.',
    usually: 'Remove the `*`. If you expected a reference, check the type: a `for x in v` loop gives owned items, while `for x in &v` gives references.',
  },
  E0615: {
    name: 'attempted to take value of method',
    means: 'You wrote `x.name` without parentheses, but `name` is a method, not a field. Methods are called with `()` even when they take no arguments.',
    usually: 'Call it: `x.name()`. If you really want a field, check the struct definition: the name you wrote belongs to a method.',
    why: "`acct.deposit` with no parentheses names a method, not a field, and Rust does not make a \"bound method\" from it. In Go (`f := acct.Deposit`), Python and JavaScript, that expression gives a function tied to the value. In Rust a method is a function whose first parameter is the receiver.",
    fixes: [
      "Call it: `acct.deposit(5)`.",
      "To pass the method around, use its path: `Account::deposit`, called as `Account::deposit(&mut acct, 5)`.",
      "Or wrap it in a closure: `let mut f = |n| acct.deposit(n);`.",
    ],
  },
  E0616: {
    name: 'field is private',
    means: 'You read or wrote a field that is private to its module.',
    usually: 'Add a method that exposes what callers need (`acct.balance()`), or make the field `pub` if outside code really should touch it directly.',
  },
  E0658: {
    name: 'use of unstable feature',
    means: 'The code uses a feature that exists in the compiler but has not been stabilised, so it is only available on the nightly toolchain (for example `try { }` blocks).',
    usually: 'Use a stable alternative (a closure that returns a `Result`, or a helper function and `?`), or opt into nightly only if you really need the feature.',
  },
  E0716: {
    name: 'temporary value dropped while borrowed',
    means: 'A temporary value was freed at the end of its statement while a reference to it was still being used afterwards.',
    usually: 'Give the value a name with `let` so it lives as long as the reference does, or keep the whole chain in one statement.',
  },
}
