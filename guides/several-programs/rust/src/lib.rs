// Shared code. Every program in this package can use it as `tools::...`.
pub fn banner(program: &str) -> String {
    format!("== {program} ==")
}
