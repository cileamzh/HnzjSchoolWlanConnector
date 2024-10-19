use crate::meb::ToVec;

/// struct HttpResponse is used to represent a httpresponse
pub struct HttpResponse {
    pub protocal: String,
    pub status: String,
    pub header: Vec<String>,
    pub body: String,
    pub binary: Vec<u8>,
}

impl HttpResponse {
    pub fn new() -> Self {
        HttpResponse {
            protocal: String::new(),
            status: String::new(),
            header: Vec::new(),
            body: String::new(),
            binary: Vec::new(),
        }
    }

    pub fn set_status(&mut self, status: &str) {
        self.status = status.to_owned();
    }

    pub fn set_header(&mut self, header: &str) {
        self.header.push(header.to_owned());
    }

    pub fn set_protocal(&mut self, protocal: &str) {
        self.protocal = protocal.to_owned();
    }

    pub fn set_body(&mut self, body: &str) {
        self.body = body.to_owned();
    }
}

impl ToVec for HttpResponse {
    fn to_vec_u8(&self) -> Vec<u8> {
        let mut all: Vec<u8> = Vec::new();
        all.append(&mut self.protocal.to_vec_u8());
        all.append(&mut " ".to_vec_u8());
        all.append(&mut self.status.to_vec_u8());
        for header in &self.header {
            all.append(&mut "\r\n".to_vec_u8());
            all.append(&mut header.to_vec_u8());
        }
        all.append(&mut "\r\n\r\n".to_vec_u8());
        all.append(&mut self.body.to_vec_u8());
        all.append(&mut self.binary.to_vec());
        all
    }
}
