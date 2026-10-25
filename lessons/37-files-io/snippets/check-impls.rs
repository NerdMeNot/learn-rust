use std::fs::File;
use std::io::{self, BufRead, BufReader, Cursor, Read, Seek, Write};
use std::marker::PhantomData;

/// `has!(Trait, Type)` is true when `Type` implements `Trait`: an inherent constant that only
/// exists under the bound beats the trait constant that is always there.
macro_rules! has {
    ($tr:path, $t:ty) => {{
        #[allow(dead_code)]
        trait No {
            const YES: bool = false;
        }
        impl<T: ?Sized> No for T {}
        struct P<T: ?Sized>(PhantomData<T>);
        #[allow(dead_code)]
        impl<T: ?Sized + $tr> P<T> {
            const YES: bool = true;
        }
        <P<$t>>::YES
    }};
}

/// Read, Write, BufRead, Seek for one type.
macro_rules! row {
    ($t:ty) => {
        [
            has!(Read, $t),
            has!(Write, $t),
            has!(BufRead, $t),
            has!(Seek, $t),
        ]
    };
}

fn main() {
    //                          Read   Write  BufRead Seek
    assert_eq!(row!(File), [true, true, false, true]);
    assert_eq!(row!(BufReader<File>), [true, false, true, true]);
    assert_eq!(row!(&[u8]), [true, false, true, false]);
    assert_eq!(row!(Cursor<Vec<u8>>), [true, true, true, true]);
    assert_eq!(row!(Vec<u8>), [false, true, false, false]);
    assert_eq!(row!(io::Stdin), [true, false, false, false]);
    assert_eq!(row!(io::StdinLock<'static>), [true, false, true, false]);
    assert_eq!(row!(io::Stdout), [false, true, false, false]);
    println!("ok");
}
