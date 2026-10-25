#[derive(Debug, PartialEq)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub retries: u32,
}

#[derive(Debug, PartialEq)]
pub enum BuildError {
    MissingUrl,
    EmptyUrl,
    UnknownMethod(String),
}

/// Collects settings; `build` checks them all and makes the `Request`.
pub struct RequestBuilder {
    url: Option<String>,
    method: String,
    retries: u32,
}

impl RequestBuilder {
    /// Defaults: method "GET", 3 retries, no url yet.
    pub fn new() -> RequestBuilder {
        RequestBuilder {
            url: None,
            method: "GET".to_string(),
            retries: 3,
        }
    }

    pub fn url(mut self, url: &str) -> RequestBuilder {
        self.url = Some(url.to_string());
        self
    }

    /// Any case is accepted; the built request holds it in upper case.
    pub fn method(mut self, method: &str) -> RequestBuilder {
        self.method = method.to_string();
        self
    }

    pub fn retries(mut self, retries: u32) -> RequestBuilder {
        self.retries = retries;
        self
    }

    /// Errors: no url was set (MissingUrl); the url is blank after trimming (EmptyUrl);
    /// the method is not GET, POST, PUT or DELETE in any case (UnknownMethod, as given).
    pub fn build(self) -> Result<Request, BuildError> {
        let url = self.url.ok_or(BuildError::MissingUrl)?;
        if url.trim().is_empty() {
            return Err(BuildError::EmptyUrl);
        }
        let upper = self.method.to_uppercase();
        if !["GET", "POST", "PUT", "DELETE"].contains(&upper.as_str()) {
            return Err(BuildError::UnknownMethod(self.method));
        }
        Ok(Request {
            url,
            method: upper,
            retries: self.retries,
        })
    }
}

fn main() {
    let r = RequestBuilder::new().url("http://a").build().unwrap();
    assert_eq!(
        r,
        Request {
            url: "http://a".into(),
            method: "GET".into(),
            retries: 3
        }
    );

    let r = RequestBuilder::new()
        .retries(0)
        .method("post")
        .url("http://b")
        .build()
        .unwrap();
    assert_eq!(
        r,
        Request {
            url: "http://b".into(),
            method: "POST".into(),
            retries: 0
        }
    );

    // the last call wins
    let r = RequestBuilder::new()
        .url("http://a")
        .url("http://c")
        .retries(1)
        .retries(2)
        .build()
        .unwrap();
    assert_eq!((r.url.as_str(), r.retries), ("http://c", 2));

    assert_eq!(
        RequestBuilder::new().retries(5).build(),
        Err(BuildError::MissingUrl)
    );
    assert_eq!(
        RequestBuilder::new().url("   ").build(),
        Err(BuildError::EmptyUrl)
    );
    assert_eq!(
        RequestBuilder::new()
            .url("http://a")
            .method("fetch")
            .build(),
        Err(BuildError::UnknownMethod("fetch".into()))
    );
    // only GET, POST, PUT and DELETE are known: other words are not
    assert_eq!(
        RequestBuilder::new()
            .url("http://a")
            .method("patch")
            .build(),
        Err(BuildError::UnknownMethod("patch".into()))
    );
    let r = RequestBuilder::new()
        .url("http://a")
        .method("put")
        .build()
        .unwrap();
    assert_eq!(r.method, "PUT");

    // a setter does not judge: a bad method that is replaced before `build` is fine
    let r = RequestBuilder::new()
        .method("fetch")
        .method("Delete")
        .url("http://a")
        .build()
        .unwrap();
    assert_eq!(r.method, "DELETE");
    println!("all checks passed");
}
