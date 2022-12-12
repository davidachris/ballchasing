use std::env;

#[derive(Debug)]
struct Properties {
    authorization: Option<String>,
}

impl Properties {
    fn new() -> Properties {
        let token = match env::var("BALLCHASING_TOKEN") {
            Ok(key) => Some(key),
            Err(_) => None,
        };
        Properties {
            authorization: token,
        }
    }
}
