use super::method::Method;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Request {
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        let s = String::from_utf8_lossy(&data[..]);
        let mut lines = s.lines();
        if let Some(line_data) = lines.next() {
            let mut parts = line_data.split_whitespace();
            let method = Method::from_str(parts.next().unwrap_or("UNKNOWN"));
            let path = parts.next().unwrap_or("/").to_string();

            let mut headers = Vec::new();

            for line in lines {
                if line.is_empty() {
                    break;
                }
                let mut parts = line.splitn(2, ':');
                if let Some(key) = parts.next() {
                    if let Some(value) = parts.next() {
                        headers.push((key.trim().to_string(), value.trim().to_string()))
                    }
                }
            }

            let mut body = String::new();
            if let Some(n) = s.find("\r\n\r\n") {
                body = s[(n + 4)..].trim().trim_matches('\0').to_string();
            }
            Some(Self {
                method,
                path,
                headers,
                body,
            })
        } else {
            None
        }
    }
}
