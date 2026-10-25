use std::io::{self, BufWriter, Write};

/// A destination that only counts how many times it is asked to write.
struct Counting {
    calls: usize,
    bytes: usize,
}

impl Write for Counting {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.calls += 1;
        self.bytes += buf.len();
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut raw = Counting { calls: 0, bytes: 0 };
    for i in 0..1000 {
        writeln!(raw, "line {i}")?;
    }
    println!(
        "unbuffered: {} bytes in {} write calls",
        raw.bytes, raw.calls
    );

    let mut w = BufWriter::new(Counting { calls: 0, bytes: 0 });
    println!("capacity: {}", w.capacity());
    for i in 0..1000 {
        writeln!(w, "line {i}")?;
    }
    println!(
        "before flush: {} calls, {} bytes handed over, {} bytes waiting",
        w.get_ref().calls,
        w.get_ref().bytes,
        w.buffer().len()
    );
    w.flush()?;
    println!(
        "after flush: {} bytes in {} write calls",
        w.get_ref().bytes,
        w.get_ref().calls
    );
    Ok(())
}
