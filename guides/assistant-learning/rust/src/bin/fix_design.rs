// Change the design: the history is its own type, owned by the caller and
// passed in. The playlist only reads its songs.
struct History {
    lines: Vec<String>,
}

impl History {
    fn record(&mut self, line: String) {
        let n = self.lines.len() + 1;
        self.lines.push(format!("{n}. {line}"));
    }
}

struct Playlist {
    songs: Vec<String>,
}

impl Playlist {
    fn play_all(&self, history: &mut History) {
        for song in &self.songs {
            history.record(format!("played {song}"));
        }
    }
}

fn main() {
    let p = Playlist {
        songs: vec!["intro".to_string(), "theme".to_string()],
    };
    let mut history = History { lines: Vec::new() };
    p.play_all(&mut history);
    println!("{:?}", history.lines);
}
