use std::fs;
use std::process::Command;

// For each name, compile `let _ = NAME;` and look at what the compiler says. Error
// E0425 ("cannot find value") means the name is not in scope without a `use`. E0423
// means a type or trait with that name is in scope (it just is not a value), and no
// error means a value of that name is in scope.
const NAMES: &str = "Vec String Box Option Result Some None Ok Err Clone Copy Drop Default \
    Iterator IntoIterator From Into TryFrom TryInto AsRef ToString ToOwned FromIterator \
    PartialEq Ord Send Sync Fn drop Future HashMap HashSet Rc Arc Display Ordering fmt io";

fn main() {
    let mut source = String::new();
    for name in NAMES.split_whitespace() {
        source += &format!("fn probe_{name}() {{ let _ = {name}; }}\n");
    }
    let dir = std::env::temp_dir().join("modules-prelude");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("probe.rs"), source).unwrap();
    let out = Command::new("rustc")
        .current_dir(&dir)
        .args(["--edition", "2024", "--crate-type=lib", "--emit=metadata"])
        .args(["-A", "warnings", "--error-format=short", "probe.rs"])
        .output()
        .unwrap();
    let errors = String::from_utf8_lossy(&out.stderr);
    let (mut free, mut needs_use) = (Vec::new(), Vec::new());
    for name in NAMES.split_whitespace() {
        if errors.contains(&format!("error[E0425]: cannot find value `{name}`")) {
            needs_use.push(name);
        } else {
            free.push(name);
        }
    }
    println!("usable without a `use`: {}", free.join(" "));
    println!("need a `use`: {}", needs_use.join(" "));
}
