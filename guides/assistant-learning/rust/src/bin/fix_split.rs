struct Playlist {
    songs: Vec<String>,
    history: Vec<String>,
}

// Split the borrow: the helper takes only the field it changes.
fn record(history: &mut Vec<String>, line: String) {
    let n = history.len() + 1;
    history.push(format!("{n}. {line}"));
}

impl Playlist {
    fn play_all(&mut self) {
        for song in &self.songs {
            record(&mut self.history, format!("played {song}"));
        }
    }
}

fn main() {
    let mut p = Playlist {
        songs: vec!["intro".to_string(), "theme".to_string()],
        history: Vec::new(),
    };
    p.play_all();
    println!("{:?}", p.history);
}
