struct Request {
    url: String,
    retries: u32,
    timeout_secs: u32,
    headers: Vec<String>,
}

impl Request {
    fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            retries: 0,
            timeout_secs: 30,
            headers: Vec::new(),
        }
    }

    fn retries(mut self, n: u32) -> Self {
        self.retries = n;
        self
    }

    fn timeout(mut self, secs: u32) -> Self {
        self.timeout_secs = secs;
        self
    }

    fn header(mut self, line: &str) -> Self {
        self.headers.push(line.to_string());
        self
    }
}

fn main() {
    let req = Request::new("https://example.com")
        .retries(3)
        .header("Accept: text/plain")
        .header("X-Id: 7")
        .timeout(5);
    println!("url: {}", req.url);
    println!("retries: {}", req.retries);
    println!("timeout: {}", req.timeout_secs);
    println!("headers: {:?}", req.headers);

    // Adding a header only sometimes means putting the value back:
    let verbose = true;
    let mut req = Request::new("https://example.com");
    if verbose {
        req = req.header("X-Debug: 1");
    }
    println!("headers: {:?}", req.headers);
}
