struct Playlist {
    songs: Vec<String>,
    history: Vec<String>,
}

impl Playlist {
    fn record(&mut self, line: String) {
        let n = self.history.len() + 1;
        self.history.push(format!("{n}. {line}"));
    }

    // Take an index: each line borrows self.songs only while format! runs.
    fn play_all(&mut self) {
        for i in 0..self.songs.len() {
            let line = format!("played {}", self.songs[i]);
            self.record(line);
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
