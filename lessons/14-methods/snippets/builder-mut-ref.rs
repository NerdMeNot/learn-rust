struct Request {
    url: String,
    retries: u32,
    headers: Vec<String>,
}

struct RequestBuilder {
    url: String,
    retries: u32,
    headers: Vec<String>,
}

impl RequestBuilder {
    fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            retries: 0,
            headers: Vec::new(),
        }
    }

    fn retries(&mut self, n: u32) -> &mut Self {
        self.retries = n;
        self
    }

    fn header(&mut self, line: &str) -> &mut Self {
        self.headers.push(line.to_string());
        self
    }

    fn build(&self) -> Request {
        Request {
            url: self.url.clone(),
            retries: self.retries,
            headers: self.headers.clone(),
        }
    }
}

fn main() {
    // One expression: the temporary builder lives until the end of the statement.
    let one = RequestBuilder::new("https://example.com")
        .retries(3)
        .header("Accept: text/plain")
        .build();
    println!("one:  {} {} {:?}", one.url, one.retries, one.headers);

    // Step by step: the builder is a `let mut` variable, and a method call does
    // not need its result put back.
    let verbose = true;
    let mut b = RequestBuilder::new("https://example.com");
    b.retries(1);
    if verbose {
        b.header("X-Debug: 1");
    }
    let two = b.build();
    println!("two:  {} {} {:?}", two.url, two.retries, two.headers);
}
