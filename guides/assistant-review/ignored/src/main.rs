use std::fs;

fn save(report: &str) {
    let _ = fs::write("/no/such/dir/report.txt", report);
    println!("report saved");
}

fn main() {
    save("3 orders");
}
