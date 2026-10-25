// Backs the fragment-specifier table: one macro per kind, asserting what it captured.
macro_rules! m_block {
    ($b:block) => {
        stringify!($b)
    };
}
macro_rules! m_stmt {
    ($s:stmt) => {
        stringify!($s)
    };
}
macro_rules! m_item {
    ($i:item) => {
        stringify!($i)
    };
}
macro_rules! m_path {
    ($p:path) => {
        stringify!($p)
    };
}
macro_rules! m_lifetime {
    ($l:lifetime) => {
        stringify!($l)
    };
}
macro_rules! m_meta {
    ($m:meta) => {
        stringify!($m)
    };
}
macro_rules! m_vis {
    ($v:vis struct $name:ident) => {
        stringify!($v)
    };
}

fn main() {
    assert_eq!(m_block!({ 1 + 1 }), "{ 1 + 1 }");
    assert_eq!(m_stmt!(let x = 5), "let x = 5");
    assert_eq!(
        m_item!(
            fn f() {}
        ),
        "fn f() {}"
    );
    assert_eq!(
        m_path!(std::collections::HashMap),
        "std::collections::HashMap"
    );
    assert_eq!(m_lifetime!('a), "'a");
    assert_eq!(m_meta!(derive(Debug)), "derive(Debug)");
    assert_eq!(m_vis!(pub(crate) struct S), "pub(crate)");
    assert_eq!(m_vis!(struct S), "");
    println!("fragment kinds checked");
}
