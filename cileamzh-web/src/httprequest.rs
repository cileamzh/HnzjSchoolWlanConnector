use std::collections::HashMap;

use crate::meb::ToVec;

pub struct HttpRequest {
    pub params: String,
    pub path: String,
    pub method: String,
    pub protocol: String,
    pub header: Vec<String>,
    pub body: String,
    pub binary: Vec<u8>,
    pub map: HashMap<String, String>,
}
impl HttpRequest {
    pub fn new() -> Self {
        Self {
            params: String::new(),
            path: String::new(),
            method: String::new(),
            protocol: String::new(),
            header: Vec::new(),
            body: String::new(),
            binary: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn from(buf: Vec<u8>) -> Self {
        let parten = "\r\n\r\n".as_bytes();
        let mut req = Self {
            params: String::new(),
            path: String::new(),
            method: String::new(),
            protocol: String::new(),
            header: Vec::new(),
            body: String::new(),
            binary: Vec::new(),
            map: HashMap::new(),
        };

        let r = split_buf(buf, parten.to_vec());

        let head = String::from_utf8_lossy(&r[0]);

        let mut result = head.split("\r\n");

        let fl: Vec<&str> = result.next().unwrap_or("").split(" ").collect();

        let header: Vec<String> = result.map(|s| s.to_owned()).collect();

        req.body = String::from_utf8_lossy(&r[1]).to_string();
        req.binary = r[1].clone();

        req.method = fl[0].to_owned();
        req.path = fl[1].split("?").nth(0).unwrap_or(fl[1]).to_owned();
        req.params = fl[1].split("?").nth(1).unwrap_or("").to_owned();
        req.protocol = fl[2].to_owned();

        req.header = header;
        req
    }

    pub fn cookies(&mut self, cookie: &str) {
        self.header.push(format!("Cookie: {cookie}"));
    }

    pub fn push_header(&mut self, header: &str) {
        self.header.push(header.to_string());
    }

    pub fn body(&mut self, body: &str) {
        self.body.push_str(body);
    }
}

impl ToVec for HttpRequest {
    fn to_vec_u8(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.append(&mut self.method.to_vec_u8());
        buf.append(&mut " ".to_vec_u8());
        buf.append(&mut self.path.to_vec_u8());
        buf.append(&mut self.params.to_vec_u8());
        buf.append(&mut " ".to_vec_u8());
        buf.append(&mut self.protocol.to_vec_u8());
        for head in &self.header {
            buf.append(&mut format!("\r\n{}", head).to_vec_u8());
        }
        buf.append(&mut "\r\n\r\n".to_vec_u8());
        buf.append(&mut self.body.to_vec_u8());
        buf.append(&mut self.binary.to_vec());
        buf.append(&mut "\r\n0\r\n\r\n".to_vec_u8());
        buf
    }
}

fn split_buf(buf: Vec<u8>, pattern: Vec<u8>) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    let mut start = 0;

    while let Some(pos) = buf[start..]
        .windows(pattern.len())
        .position(|window| window == pattern.as_slice())
    {
        let end = start + pos;
        if start < end {
            result.push(buf[start..end].to_vec()); // Push the chunk before the pattern
        }
        start = end + pattern.len(); // Move past the pattern
    }

    if start < buf.len() {
        result.push(buf[start..].to_vec()); // Push the remaining part after the last pattern
    }
    result
}
fn _contains_array(outer: Vec<u8>, inner: &[u8]) -> bool {
    // Check if the inner array is longer than the outer array
    if inner.len() > outer.len() {
        return false;
    }

    // Check for the inner array in the outer array
    for window in outer.windows(inner.len()) {
        if window == inner {
            return true;
        }
    }
    false
}
