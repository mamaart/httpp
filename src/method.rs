#[derive(Debug)]
pub enum Method {
    POST,
    GET,
    DELETE,
    PUT,
    UNKNOWN,
}

impl Method {
    pub fn from_str(s: &str) -> Self {
        match s {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "DELETE" => Self::DELETE,
            "PUT" => Self::PUT,
            _ => Self::UNKNOWN,
        }
    }
}
