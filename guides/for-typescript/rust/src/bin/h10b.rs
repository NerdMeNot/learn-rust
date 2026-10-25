struct Options {
    retries: u32,
    verbose: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options { retries: 3, verbose: false }
    }
}

fn connect(opts: Options) {
    println!("retries {}, verbose {}", opts.retries, opts.verbose);
}

fn main() {
    connect(Options::default());
    connect(Options { verbose: true, ..Default::default() });
}
