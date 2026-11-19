struct Playlist {
    songs: Vec<String>,
    history: Vec<String>,
}

impl Playlist {
    fn record(&mut self, line: String) {
        let n = self.history.len() + 1;
        self.history.push(format!("{n}. {line}"));
    }

    fn play_all(&mut self) {
        for song in &self.songs {
            self.record(format!("played {song}"));
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
