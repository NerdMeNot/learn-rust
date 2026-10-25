// Backs the table of Result methods in the lesson.
fn main() {
    let ok: Result<i32, String> = Ok(4);
    let err: Result<i32, String> = Err("bad".to_string());

    assert_eq!(ok.clone().map(|n| n * 10), Ok(40));
    assert_eq!(err.clone().map(|n| n * 10), Err("bad".to_string()));
    assert_eq!(ok.clone().map_err(|e| e.len()), Ok(4));
    assert_eq!(err.clone().map_err(|e| e.len()), Err(3));
    assert_eq!(
        ok.clone().and_then(|n| if n > 5 {
            Ok(n)
        } else {
            Err("small".to_string())
        }),
        Err("small".to_string())
    );
    assert_eq!(
        err.clone().and_then(|n| Ok::<i32, String>(n + 1)),
        Err("bad".to_string())
    );
    assert_eq!(ok.clone().unwrap_or(0), 4);
    assert_eq!(err.clone().unwrap_or(0), 0);
    assert_eq!(err.clone().unwrap_or_else(|e| e.len() as i32), 3);
    assert_eq!(err.clone().unwrap_or_default(), 0);
    assert_eq!(ok.clone().ok(), Some(4));
    assert_eq!(err.clone().ok(), None);
    assert_eq!(err.clone().err(), Some("bad".to_string()));
    assert_eq!(Some(4).ok_or("none"), Ok(4));
    assert_eq!(None::<i32>.ok_or("none"), Err("none"));
    assert_eq!(ok.clone().unwrap_or_else(|e| e.len() as i32), 4);
    assert_eq!(ok.clone().unwrap_or_default(), 4);
    assert_eq!(ok.clone().err(), None);
    assert!(ok.is_ok() && !ok.is_err());
    assert!(err.is_err() && !err.is_ok());
    println!("all checks passed");
}
