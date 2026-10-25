// Backs the conversion table. Not shown on the page.
use std::error::Error;

macro_rules! has {
    ($t:ty, $tr:path) => {{
        #[allow(dead_code)]
        trait No {
            const YES: bool = false;
        }
        impl<T: ?Sized> No for T {}
        struct W<T: ?Sized>(std::marker::PhantomData<T>);
        #[allow(dead_code)]
        impl<T: ?Sized + $tr> W<T> {
            const YES: bool = true;
        }
        <W<$t>>::YES
    }};
}

// A From of your own is only a promise not to fail: this one loses data.
struct Lossy(u8);
impl From<i64> for Lossy {
    fn from(n: i64) -> Lossy {
        Lossy(n as u8)
    }
}

fn main() {
    assert_eq!(Lossy::from(300).0, 44);
    // std implements From between numbers only when no value can be lost
    assert!(has!(i64, From<i32>) && has!(u32, From<u8>) && has!(f64, From<i32>));
    assert!(has!(i32, From<i16>) && has!(i32, From<u16>) && has!(u64, From<u32>));
    assert!(!has!(i32, From<i64>) && !has!(i32, From<u32>) && !has!(u8, From<i8>));
    assert!(!has!(f64, From<i64>) && !has!(i32, From<f64>) && !has!(u32, From<i32>));

    // &str -> String
    let s = "hi";
    assert_eq!(String::from(s), "hi");
    let a: String = s.into();
    assert_eq!(a, "hi");
    assert_eq!(s.to_string(), "hi");
    assert_eq!(s.to_owned(), "hi");

    // String -> &str is a borrow
    let owned = String::from("hi");
    let b: &str = &owned;
    assert_eq!(b, owned.as_str());

    // widening: From exists and cannot fail
    assert_eq!(i64::from(7_i32), 7_i64);
    let w: i64 = 7_i32.into();
    assert_eq!(w, 7);
    assert_eq!(u32::from(7_u8), 7);
    assert_eq!(f64::from(7_i32), 7.0);
    assert_eq!(f64::from(1.5_f32), 1.5);
    assert_eq!(i32::from(true), 1);

    // narrowing: TryFrom, or `as` which never fails and may change the value
    assert!(i32::try_from(5_000_000_000_i64).is_err());
    assert_eq!(i32::try_from(5_i64), Ok(5));
    assert_eq!(5_000_000_000_i64 as i32, 705_032_704);
    assert_eq!(u8::try_from(-1_i32).is_err(), true);
    assert_eq!(-1_i32 as u8, 255);

    // char and numbers
    assert_eq!(char::from(65_u8), 'A');
    assert_eq!(u32::from('A'), 65);
    assert_eq!(char::from_u32(0x41), Some('A'));
    assert_eq!(char::from_u32(0xD800), None);
    assert_eq!(char::try_from(0x41_u32), Ok('A'));
    assert!(char::try_from(0xD800_u32).is_err());
    assert_eq!(u8::try_from('A'), Ok(65));
    assert!(u8::try_from('€').is_err());
    assert_eq!(u8::try_from('é'), Ok(233));

    // text <-> numbers
    assert_eq!("42".parse::<i32>(), Ok(42));
    assert!("4x".parse::<i32>().is_err());
    assert_eq!(42.to_string(), "42");

    // lists
    let v = Vec::from([1, 2, 3]);
    assert_eq!(v, vec![1, 2, 3]);
    let arr: [i32; 3] = <[i32; 3]>::try_from(v.clone()).unwrap();
    assert_eq!(arr, [1, 2, 3]);
    assert!(<[i32; 2]>::try_from(v.clone()).is_err());
    assert_eq!(v[..2].to_vec(), vec![1, 2]);
    assert_eq!(String::from("hi").into_bytes(), vec![104, 105]);
    assert_eq!(String::from_utf8(vec![104, 105]), Ok(String::from("hi")));
    assert!(String::from_utf8(vec![0xFF]).is_err());

    // Option and Box<dyn Error>
    let o: Option<i32> = 5.into();
    assert_eq!(o, Some(5));
    let e: Box<dyn Error> = "failed".into();
    assert_eq!(e.to_string(), "failed");

    // `as` between numbers always compiles; it truncates, wraps or rounds
    assert_eq!(3.99_f64 as i32, 3);
    assert_eq!(i64::MAX as f64, 9.223372036854776e18);
    println!("all checks passed");
}
