use std::fs;
use std::os::unix::fs::symlink;

fn main() -> std::io::Result<()> {
    let dir = std::env::temp_dir().join("lr-symlink");
    fs::create_dir_all(&dir)?;
    fs::write(dir.join("real.txt"), "hello")?;
    symlink("real.txt", dir.join("link"))?;

    let followed = fs::metadata(dir.join("link"))?;
    let itself = fs::symlink_metadata(dir.join("link"))?;
    println!(
        "metadata(link): is_file {}, len {}",
        followed.is_file(),
        followed.len()
    );
    println!("symlink_metadata(link): is_symlink {}", itself.is_symlink());
    println!("read_link: {:?}", fs::read_link(dir.join("link"))?);
    println!(
        "read_to_string(link): {:?}",
        fs::read_to_string(dir.join("link"))?
    );

    fs::remove_file(dir.join("real.txt"))?;
    let link = dir.join("link");
    println!(
        "dangling: exists {}, but symlink_metadata is_ok {}",
        link.exists(),
        fs::symlink_metadata(&link).is_ok()
    );

    fs::remove_dir_all(&dir)
}
