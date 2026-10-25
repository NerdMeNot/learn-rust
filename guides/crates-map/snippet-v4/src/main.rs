use uuid::Uuid;

fn main() {
    let id = Uuid::new_v4();
    println!("version {}", id.get_version_num());
}
