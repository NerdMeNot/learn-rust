use std::env;
use std::process::Command;

fn report() {
    // an unset variable is an Err you can see
    match env::var("RBC_LEVEL") {
        Ok(value) => println!("  RBC_LEVEL = {value:?}"),
        Err(e) => println!("  RBC_LEVEL: {e} / {e:?}"),
    }
    println!("  var_os: {:?}", env::var_os("RBC_LEVEL"));

    // a default for a missing variable: it does not replace an empty one
    let level = env::var("RBC_LEVEL").unwrap_or_else(|_| "info".to_string());
    println!("  level: {level:?}");
    let level = env::var("RBC_LEVEL")
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "info".to_string());
    println!("  level, empty counts as missing: {level:?}");

    // a number that must parse; anything else gets the default
    let workers: usize = env::var("RBC_WORKERS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(4);
    println!("  workers: {workers}");
}

fn main() {
    if env::args().nth(1).as_deref() == Some("--child") {
        report();
        return;
    }

    // start copies of this program with a chosen environment, to see each case
    let me = env::current_exe().expect("the path of this program");
    for (label, level, workers) in [("unset", None, None), ("empty", Some(""), Some("many"))] {
        let mut child = Command::new(&me);
        child
            .arg("--child")
            .env_remove("RBC_LEVEL")
            .env_remove("RBC_WORKERS");
        if let Some(value) = level {
            child.env("RBC_LEVEL", value);
        }
        if let Some(value) = workers {
            child.env("RBC_WORKERS", value);
        }
        let out = child.output().expect("the child should start");
        println!("RBC_LEVEL {label}:");
        print!("{}", String::from_utf8_lossy(&out.stdout));
    }

    // vars() lists every variable, in no promised order, so sort before you rely on one
    let mut names: Vec<String> = env::vars().map(|(name, _)| name).collect();
    names.sort();
    println!(
        "vars() has PATH: {}",
        names.binary_search(&"PATH".to_string()).is_ok()
    );
}
