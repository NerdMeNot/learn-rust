// Backs the table of split methods in the lesson: every row is asserted here.
macro_rules! pieces {
    ($it:expr) => {
        $it.collect::<Vec<&str>>()
    };
}

fn main() {
    assert_eq!(pieces!("a,b,c".split(',')), ["a", "b", "c"]);
    assert_eq!(pieces!("a,b,".split(',')), ["a", "b", ""]);
    assert_eq!(pieces!("a,b,".split_terminator(',')), ["a", "b"]);
    assert_eq!(pieces!("a,b,c".splitn(2, ',')), ["a", "b,c"]);
    assert_eq!(pieces!("a,b,c".rsplitn(2, ',')), ["c", "a,b"]);
    assert_eq!(pieces!("a,b,c".rsplit(',')), ["c", "b", "a"]);
    assert_eq!(pieces!("a\nb\n".split_inclusive('\n')), ["a\n", "b\n"]);
    assert_eq!(pieces!(" a  b\t".split_whitespace()), ["a", "b"]);
    assert_eq!(pieces!("a\r\nb\n".lines()), ["a", "b"]);
    assert_eq!("k=v=w".split_once('='), Some(("k", "v=w")));
    assert_eq!("k=v=w".rsplit_once('='), Some(("k=v", "w")));
    assert_eq!("kvw".split_once('='), None);
    println!("all table rows hold");
}
