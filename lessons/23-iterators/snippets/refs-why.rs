// A photo is big: its pixels live on the heap. Cloning one prints, so you can see it.
struct Photo {
    name: String,
    pixels: Vec<u8>,
}

impl Clone for Photo {
    fn clone(&self) -> Self {
        println!(
            "  cloning {}: copying {} bytes",
            self.name,
            self.pixels.len()
        );
        Photo {
            name: self.name.clone(),
            pixels: self.pixels.clone(),
        }
    }
}

fn main() {
    let photos = vec![
        Photo {
            name: "beach".into(),
            pixels: vec![0; 3_000_000],
        },
        Photo {
            name: "city".into(),
            pixels: vec![0; 5_000_000],
        },
        Photo {
            name: "dog".into(),
            pixels: vec![0; 2_000_000],
        },
    ];

    println!("iter(): borrow each photo");
    let total: usize = photos.iter().map(|p| p.pixels.len()).sum();
    println!("  total size: {total} bytes, and nothing was copied");
    println!("  photos is still usable: {} photos", photos.len());
    println!(
        "  each item handed out: {} bytes (a &Photo), not {} (a Photo) plus its pixels",
        size_of::<&Photo>(),
        size_of::<Photo>()
    );

    println!("cloned(): an owned copy of each");
    let copies: Vec<Photo> = photos.iter().cloned().collect();
    println!("  made {} copies", copies.len());

    println!("into_iter(): take the photos themselves, no copies");
    let names: Vec<String> = photos.into_iter().map(|p| p.name).collect();
    println!("  {names:?}");
}
