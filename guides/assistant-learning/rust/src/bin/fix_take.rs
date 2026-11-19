struct Playlist {
    songs: Vec<String>,
    history: Vec<String>,
}

impl Playlist {
    fn record(&mut self, line: String) {
        let n = self.history.len() + 1;
        self.history.push(format!("{n}. {line}"));
    }

    // mem::take: move the songs out (an empty Vec is left in their place),
    // loop, then put them back.
    fn play_all(&mut self) {
        let songs = std::mem::take(&mut self.songs);
        for song in &songs {
            self.record(format!("played {song}"));
        }
        self.songs = songs;
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
