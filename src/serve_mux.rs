use std::io::prelude::Read;
use std::{
    collections::HashMap,
    net::{TcpListener, TcpStream},
};

use super::{request::Request, response_writer::ResponseWriter};

pub type HandlerFunc = fn(&mut ResponseWriter, &Request);

pub struct ServeMux {
    routes: HashMap<String, HandlerFunc>,
}

impl ServeMux {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn handle_func(&mut self, path: &str, f: HandlerFunc) {
        self.routes.insert(path.to_string(), f);
    }

    pub fn listen_and_serve(&mut self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();

        for s in listener.incoming() {
            let _s = s.unwrap();
            self.handle_conn(_s)
        }
    }

    fn handle_conn(&mut self, mut stream: TcpStream) {
        let mut buf = [0; 1024];
        stream.read(&mut buf).unwrap();
        let req = Request::from_bytes(&buf).unwrap();

        let mut writer = ResponseWriter::new(stream);
        if let Some(handler) = self.routes.get(&req.path) {
            handler(&mut writer, &req)
        }
    }
}
