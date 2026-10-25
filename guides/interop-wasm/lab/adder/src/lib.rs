// A WebAssembly function can take and return only numbers (i32, i64, f32, f64).
#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn fib(n: u32) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

// The host (JavaScript) supplies this function when it creates the module.
#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn host_log(ptr: *const u8, len: usize);
}

fn log(msg: &str) {
    unsafe { host_log(msg.as_ptr(), msg.len()) }
}

// Text and arrays cross as a pointer and a length into the module's linear memory.
// The host asks the module for room, writes the bytes there, and calls with the address.
#[unsafe(no_mangle)]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    Box::into_raw(vec![0u8; len].into_boxed_slice()) as *mut u8
}

/// # Safety
/// `ptr` and `len` must be exactly what `alloc` (or `shout`) handed out, once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, len: usize) {
    drop(unsafe { Box::from_raw(std::ptr::slice_from_raw_parts_mut(ptr, len)) });
}

/// # Safety
/// `ptr..ptr+len` must be bytes the host wrote into memory from `alloc`; `out_len`
/// must point at 4 writable bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shout(ptr: *const u8, len: usize, out_len: *mut usize) -> *mut u8 {
    let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
    let Ok(text) = std::str::from_utf8(bytes) else {
        log("shout: not UTF-8");
        unsafe { *out_len = 0 };
        return std::ptr::null_mut();
    };
    log(&format!("shout got {len} bytes"));
    let result = format!("{}!", text.to_uppercase()).into_bytes().into_boxed_slice();
    unsafe { *out_len = result.len() };
    Box::into_raw(result) as *mut u8
}

/// # Safety
/// `ptr..ptr+len` must be `len` i32 values in memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sum(ptr: *const i32, len: usize) -> i64 {
    unsafe { std::slice::from_raw_parts(ptr, len) }.iter().map(|&x| x as i64).sum()
}

#[unsafe(no_mangle)]
pub extern "C" fn divide(a: i32, b: i32) -> i32 {
    a / b
}
