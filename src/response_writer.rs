use super::status::Status;
use std::io::prelude::Write;
use std::{collections::HashMap, net::TcpStream};

pub struct ResponseWriter {
    status: Status,
    headers: HashMap<String, String>,
    stream: TcpStream,
}

impl ResponseWriter {
    pub fn new(conn: TcpStream) -> Self {
        Self {
            stream: conn,
            headers: HashMap::new(),
            status: Status::OK,
        }
    }

    pub fn write_header(&mut self, status: Status) {
        self.status = status;
    }

    pub fn write(&mut self, data: &[u8]) {
        let (code, status) = self.status.as_tuple();
        let mut resp = format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n",
            code,
            status,
            data.len(),
        );

        for (k, v) in &self.headers {
            resp.push_str(&format!("{}: {}\r\n", k, v));
        }
        resp.push_str("\r\n");

        self.stream.write_all(resp.as_bytes()).unwrap();
        self.stream.write_all(data).unwrap();
        self.stream.flush().unwrap();
    }
}
