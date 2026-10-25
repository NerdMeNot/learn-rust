use std::time::Duration;

struct Request {
    url: String,
    method: String,
    retries: u32,
    timeout: Duration,
}

#[derive(Debug, PartialEq)]
enum BuildError {
    MissingUrl,
}

struct RequestBuilder {
    url: Option<String>,
    method: String,
    retries: u32,
    timeout: Duration,
}

impl RequestBuilder {
    fn new() -> RequestBuilder {
        RequestBuilder {
            url: None,
            method: "GET".to_string(),
            retries: 3,
            timeout: Duration::from_secs(30),
        }
    }

    // Each setter takes `self` and returns it, so calls chain and nothing is borrowed.
    fn url(mut self, url: &str) -> RequestBuilder {
        self.url = Some(url.to_string());
        self
    }

    fn method(mut self, method: &str) -> RequestBuilder {
        self.method = method.to_string();
        self
    }

    fn retries(mut self, retries: u32) -> RequestBuilder {
        self.retries = retries;
        self
    }

    /// The one place that checks the whole value.
    fn build(self) -> Result<Request, BuildError> {
        let url = self.url.ok_or(BuildError::MissingUrl)?;
        Ok(Request {
            url,
            method: self.method,
            retries: self.retries,
            timeout: self.timeout,
        })
    }
}

fn main() {
    let ok = RequestBuilder::new()
        .url("https://example.org")
        .method("POST")
        .retries(0)
        .build();
    match &ok {
        Ok(r) => println!(
            "{} {} retries={} timeout={:?}",
            r.method, r.url, r.retries, r.timeout
        ),
        Err(e) => println!("error: {e:?}"),
    }
    let missing = RequestBuilder::new().retries(5).build();
    println!("{:?}", missing.map(|r| r.url));
}
