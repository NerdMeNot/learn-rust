#[derive(Default)]
struct Settings {
    name: String,
    retries: u32,
    verbose: bool,
    ratio: f64,
    tags: Vec<String>,
    limit: Option<u32>,
}

#[derive(Debug, Default, PartialEq)]
enum Mode {
    Fast,
    #[default]
    Safe,
}

struct Server {
    port: u16,
    workers: u32,
}

// 0 is not a useful port, so this one is written by hand.
impl Default for Server {
    fn default() -> Server {
        Server {
            port: 8080,
            workers: 4,
        }
    }
}

fn main() {
    let s = Settings::default();
    println!(
        "name {:?}, retries {}, verbose {}",
        s.name, s.retries, s.verbose
    );
    println!("ratio {}, tags {:?}, limit {:?}", s.ratio, s.tags, s.limit);
    println!(
        "mode: {:?}, fast: {}",
        Mode::default(),
        Mode::default() == Mode::Fast
    );

    let custom = Settings {
        retries: 3,
        verbose: true,
        ..Default::default()
    };
    println!(
        "retries {} verbose {} name {:?}",
        custom.retries, custom.verbose, custom.name
    );

    let server = Server::default();
    println!("port {}, workers {}", server.port, server.workers);
    let missing: Option<u32> = None;
    println!("unwrap_or_default: {}", missing.unwrap_or_default());
}
