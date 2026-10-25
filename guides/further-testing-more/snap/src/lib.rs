/// A text report of hours per project, the first column is as wide as the longest project name.
pub fn report(rows: &[(&str, f64)]) -> String {
    let width = rows.iter().map(|(name, _)| name.len()).max().unwrap_or(0);
    let mut out = String::new();
    for (name, hours) in rows {
        out.push_str(&format!("{name:<width$}  {hours:>5.1}h\n"));
    }
    let total: f64 = rows.iter().map(|(_, h)| h).sum();
    out.push_str(&format!("{:<width$}  {total:>5.1}h\n", "total"));
    out
}
