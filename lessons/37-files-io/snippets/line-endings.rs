use std::fs;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let data: &[u8] = b"a\r\nb\nc\r\n\r\nd"; // mixed endings, no newline at the end
    let lines: Vec<String> = BufReader::new(data).lines().map(|l| l.unwrap()).collect();
    println!("lines(): {lines:?}");

    let mut reader = BufReader::new(data);
    let mut line = String::new();
    let mut seen = Vec::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        seen.push(line.clone());
        line.clear(); // read_line appends, so clear it between reads
    }
    println!("read_line(): {seen:?}");

    let pieces: Vec<&[u8]> = data.split(|&b| b == b'\n').collect();
    println!(
        "split on \\n: {} pieces, first is {:?}",
        pieces.len(),
        String::from_utf8_lossy(pieces[0])
    );

    let lone: Vec<String> = BufReader::new(&b"x\ry\n"[..])
        .lines()
        .map(|l| l.unwrap())
        .collect();
    println!("a lone \\r is not a line break: {lone:?}");

    // a file is no different: read_to_string does not translate \r\n
    let path = std::env::temp_dir().join("lr-line-endings.txt");
    fs::write(&path, "a\r\nb\r\n")?;
    println!("read_to_string: {:?}", fs::read_to_string(&path)?);
    fs::remove_file(&path)
}
