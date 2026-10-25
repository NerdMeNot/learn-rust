use anyhow::{Context, Result};
use std::fs;

fn load(path: &str) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("reading {path}"))
}

fn main() {
    let e = load("missing.conf").unwrap_err();
    println!("{{}}   : {e}");
    println!("{{:#}}  : {e:#}");
    println!("{{:?}}  :\n{e:?}");
    println!("links in the chain: {}", e.chain().count());
    println!("root cause: {}", e.root_cause());
}
