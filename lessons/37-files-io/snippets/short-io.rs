use std::io::{Read, Write};

fn main() {
    // read fills at most the buffer and says how much it put there
    let mut src: &[u8] = b"0123456789";
    let mut buf = [0u8; 4];
    let mut sizes = Vec::new();
    loop {
        let n = src.read(&mut buf).unwrap();
        if n == 0 {
            break; // for a non-empty buffer, 0 means the end of the input
        }
        sizes.push(n);
    }
    println!("read() returned: {sizes:?}");

    let mut src: &[u8] = b"abc";
    println!("read into an empty buffer: {:?}", src.read(&mut []));

    let err = src.read_exact(&mut [0u8; 4]).unwrap_err();
    println!("read_exact on 3 bytes: {:?}", err.kind());

    // write may take only part of what you offer; write_all keeps going or fails
    let mut room = [0u8; 3];
    let mut dst: &mut [u8] = &mut room;
    println!("write 5 bytes into 3: {:?}", dst.write(b"hello"));

    let mut room = [0u8; 3];
    let mut dst: &mut [u8] = &mut room;
    let err = dst.write_all(b"hello").unwrap_err();
    println!("write_all 5 bytes into 3: {:?}", err.kind());
}
