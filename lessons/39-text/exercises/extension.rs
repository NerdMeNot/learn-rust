/// The extension of a file name: the text after its last dot.
/// A name with no dot, a name whose only dot is the first character (".bashrc",
/// a hidden file), and a name that ends in a dot have no extension.
/// (The stem is the part before the last dot.)
fn extension(name: &str) -> Option<&str> {
    todo!("split at the LAST dot, then reject an empty stem or an empty extension")
}

fn main() {
    assert_eq!(extension("notes.md"), Some("md"));
    assert_eq!(extension("archive.tar.gz"), Some("gz"));
    assert_eq!(extension(".hidden.txt"), Some("txt"));
    assert_eq!(extension("README"), None);
    assert_eq!(extension(".bashrc"), None);
    assert_eq!(extension("trailing."), None);
    assert_eq!(extension(""), None);
    assert_eq!(extension("résumé.pdf"), Some("pdf"));
    assert_eq!(extension("photo.ÉTÉ"), Some("ÉTÉ"));

    // The answer is a view into the name, not a copy.
    let name = String::from("data.csv");
    let ext = extension(&name).unwrap();
    assert_eq!(ext.as_ptr(), name[5..].as_ptr());
    println!("all checks passed");
}
