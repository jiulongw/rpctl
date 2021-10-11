use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::net;

pub struct Request {
    method: String,
    path: String,
}

impl Request {
    fn new(stream: &net::TcpStream) -> Option<Request> {
        let mut input = io::BufReader::new(stream.try_clone().unwrap());
        let mut line = String::new();
        if let Some(err) = input.read_line(&mut line).err() {
            eprintln!("Failed to read request line: {:?}", err);
            return None;
        }

        let parts: Vec<&str> = line.trim_end().split(' ').collect();
        Some(Request {
            path: parts[1].trim_end_matches('/').to_string(),
            method: parts[0].to_string(),
        })
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

pub struct ResponseWriter<'a> {
    output: io::BufWriter<&'a net::TcpStream>,
    status_code: u32,
    body: Vec<u8>,
}

impl<'a> ResponseWriter<'a> {
    fn new(stream: &'a net::TcpStream) -> ResponseWriter {
        let output = io::BufWriter::new(stream);
        ResponseWriter {
            output,
            status_code: 200,
            body: Vec::new(),
        }
    }

    pub fn set_status(&mut self, code: u32) {
        self.status_code = code;
    }

    pub fn body(&mut self) -> &mut Vec<u8> {
        &mut self.body
    }
}

impl Drop for ResponseWriter<'_> {
    fn drop(&mut self) {
        let message = match self.status_code {
            200 => "OK",
            204 => "NO CONTENT",
            404 => "NOT FOUND",
            405 => "METHOD NOT ALLOWED",
            _ => "INTERNAL SERVER ERROR",
        };

        if let Some(err) =
            write!(self.output, "HTTP/1.1 {} {}\r\n", self.status_code, message).err()
        {
            eprintln!("Write status failed: {:?}", err)
        }

        if let Some(err) = write!(self.output, "Content-Length: {}\r\n", self.body.len()).err() {
            eprintln!("Write content length failed: {:?}", err)
        }

        if let Some(err) = write!(self.output, "\r\n").err() {
            eprintln!("Write separator failed: {:?}", err)
        }

        if let Some(err) = self.output.write(&self.body).err() {
            eprintln!("Write body failed: {:?}", err)
        }
    }
}

pub trait Handler {
    fn handle(&self, r: &Request, w: &mut ResponseWriter);
}

pub struct Server<'a> {
    port: u16,
    handlers: HashMap<String, &'a dyn Handler>,
}

impl<'a> Server<'a> {
    pub fn new(port: u16) -> Server<'a> {
        Server {
            port,
            handlers: HashMap::new(),
        }
    }

    pub fn register(&mut self, path: &str, handler: &'a dyn Handler) {
        self.handlers.insert(path.to_string(), handler);
    }

    pub fn serve(&self) {
        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_stream(stream);
        }
    }

    fn handle_stream(&self, stream: net::TcpStream) {
        if let Some(request) = Request::new(&stream) {
            let mut writer = ResponseWriter::new(&stream);

            match self.handlers.get(&request.path) {
                Some(handler) => handler.handle(&request, &mut writer),
                None => writer.set_status(404),
            }
        }
    }
}
