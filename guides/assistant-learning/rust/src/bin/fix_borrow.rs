struct Playlist {
    songs: Vec<String>,
    history: Vec<String>,
}

impl Playlist {
    fn record(&mut self, line: String) {
        let n = self.history.len() + 1;
        self.history.push(format!("{n}. {line}"));
    }

    // Borrow for less time: build every line first. The read borrow of
    // self.songs ends at collect(), before the first `&mut self` call.
    fn play_all(&mut self) {
        let lines: Vec<String> = self.songs.iter().map(|s| format!("played {s}")).collect();
        for line in lines {
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
