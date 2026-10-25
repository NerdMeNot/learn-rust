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
    // add the fields you need
}

impl RequestBuilder {
    /// Defaults: method "GET", 3 retries, no url yet.
    pub fn new() -> RequestBuilder {
        todo!("start with the defaults")
    }

    pub fn url(self, url: &str) -> RequestBuilder {
        todo!("remember the url; checking it is build's job")
    }

    /// Any case is accepted; the built request holds it in upper case.
    pub fn method(self, method: &str) -> RequestBuilder {
        todo!("remember the method as given")
    }

    pub fn retries(self, retries: u32) -> RequestBuilder {
        todo!("remember the count")
    }

    /// Errors: no url was set (MissingUrl); the url is blank after trimming (EmptyUrl);
    /// the method is not GET, POST, PUT or DELETE in any case (UnknownMethod, as given).
    pub fn build(self) -> Result<Request, BuildError> {
        todo!("check everything here and return the Request")
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
