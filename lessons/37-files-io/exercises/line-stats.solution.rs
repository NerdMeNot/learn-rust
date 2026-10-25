use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug, PartialEq)]
struct Stats {
    /// Lines read. A last line without a newline still counts.
    lines: usize,
    /// Lines with no characters at all once the line ending is gone.
    blank: usize,
    /// The most characters (not bytes) in one line, not counting its ending.
    longest: usize,
    /// True if reading stopped at a line longer than the limit.
    stopped: bool,
}

/// Read `reader` one line at a time. It may be a file, standard input or a byte slice.
/// An unreadable line (bad UTF-8, a failing disk) is an error for the caller.
/// Stop at the first line with more than `limit` characters: count that line, then
/// read no further, so the rest of a huge file is never touched.
fn stats<R: BufRead>(reader: R, limit: usize) -> io::Result<Stats> {
    let mut found = Stats {
        lines: 0,
        blank: 0,
        longest: 0,
        stopped: false,
    };
    for line in reader.lines() {
        let line = line?;
        let chars = line.chars().count();
        found.lines += 1;
        if line.is_empty() {
            found.blank += 1;
        }
        found.longest = found.longest.max(chars);
        if chars > limit {
            found.stopped = true;
            break;
        }
    }
    Ok(found)
}

/// A reader that hands out some bytes and then fails.
struct Flaky(bool);

impl Read for Flaky {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.0 {
            return Err(io::Error::other("the disk went away"));
        }
        self.0 = true;
        let data = b"one\ntwo\n";
        buf[..data.len()].copy_from_slice(data);
        Ok(data.len())
    }
}

/// A reader that hands out its bytes and then fails. A program that reads everything before
/// it looks at a line runs into the failure; one that stops at the long line never does.
struct RunsDry<'a>(&'a [u8]);

impl Read for RunsDry<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.0.is_empty() {
            return Err(io::Error::other("read past the long line"));
        }
        let n = buf.len().min(self.0.len());
        buf[..n].copy_from_slice(&self.0[..n]);
        self.0 = &self.0[n..];
        Ok(n)
    }
}

fn check(input: &[u8], lines: usize, blank: usize, longest: usize) {
    let got = stats(input, usize::MAX).unwrap();
    assert_eq!(
        got,
        Stats {
            lines,
            blank,
            longest,
            stopped: false
        },
        "input {:?}",
        String::from_utf8_lossy(input)
    );
}

fn main() {
    check(b"", 0, 0, 0);
    check(b"one\ntwo\n", 2, 0, 3);
    check(b"a\nbc", 2, 0, 2); // no newline at the end
    check("h\u{e9}llo\r\n\r\nx\r\n".as_bytes(), 3, 1, 5); // CRLF, and \u{e9} is 2 bytes but 1 char
    check(b"   \n", 1, 0, 3); // spaces are characters
    check(b"\n\n", 2, 2, 0);

    let err = stats(&b"good\nbad \xff\nmore\n"[..], usize::MAX)
        .err()
        .expect("bad UTF-8 must be reported");
    assert_eq!(
        err.kind(),
        io::ErrorKind::InvalidData,
        "bad UTF-8 must be reported"
    );

    // BufReader with a tiny buffer: a line is longer than the buffer
    let long = "x".repeat(1000) + "\nshort\n";
    let small = BufReader::with_capacity(8, long.as_bytes());
    assert_eq!(
        stats(small, usize::MAX).unwrap(),
        Stats {
            lines: 2,
            blank: 0,
            longest: 1000,
            stopped: false
        }
    );

    let err = stats(BufReader::new(Flaky(false)), usize::MAX)
        .err()
        .expect("a failing read must be reported");
    assert_eq!(
        err.to_string(),
        "the disk went away",
        "a failing read must not be swallowed"
    );

    // the limit counts characters, not bytes, and a line of exactly `limit` is fine
    let text = "h\u{e9}llo\nsix\u{e9}\u{e9}\u{e9}\n";
    let got = stats(text.as_bytes(), 5).unwrap();
    assert_eq!(
        got,
        Stats {
            lines: 2,
            blank: 0,
            longest: 6,
            stopped: true
        },
        "5 characters are not over a limit of 5; 6 characters are"
    );

    // stop at the long line: the reader fails if it is asked for more bytes
    let data = b"ok\n\nthis line is too long\nnever looked at \xff\n";
    let got = stats(BufReader::new(RunsDry(data)), 10)
        .expect("stop at the long line; do not read the rest of the input");
    assert_eq!(
        got,
        Stats {
            lines: 3,
            blank: 1,
            longest: 21,
            stopped: true
        }
    );

    println!("all checks passed");
}
