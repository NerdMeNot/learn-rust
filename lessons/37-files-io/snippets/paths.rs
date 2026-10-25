use std::path::{Path, PathBuf};

fn main() {
    let p = Path::new("/home/ana/notes/todo.final.txt");
    println!("file_name: {:?}", p.file_name());
    println!("file_stem: {:?}", p.file_stem());
    println!("extension: {:?}", p.extension());
    println!("parent: {:?}", p.parent());
    println!("with_extension: {:?}", p.with_extension("md"));

    println!(".bashrc extension: {:?}", Path::new(".bashrc").extension());
    println!(
        "a.tar.gz extension: {:?}",
        Path::new("a.tar.gz").extension()
    );

    let mut b = PathBuf::from("/data");
    b.push("logs");
    b.push("app.log");
    println!("built: {}", b.display());
    println!(
        "join(absolute) replaces: {:?}",
        Path::new("/data").join("/etc/hosts")
    );
    println!(
        "starts_with is by component: {}",
        Path::new("/data/logs").starts_with("/dat")
    );
    println!(
        "components: {:?}",
        Path::new("/data/./logs/../app.log")
            .components()
            .collect::<Vec<_>>()
    );
}
