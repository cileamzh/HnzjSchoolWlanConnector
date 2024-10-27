use std::{env::current_exe, fs::read_to_string, process::Command, thread, time::Duration};

static MAX_RETRY: u8 = 5;
static STUDENT_WLAN: &str = "HNZJ-Student";
static TEACHER_WLAN: &str = "HNZJ-Teacher";

fn main() -> std::io::Result<()> {
    let mut account: &str = "";
    let mut password: &str = "";
    let mut identity: &str = "";
    let mut operator: &str = "";
    let stp = current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
        .join("setting.config");

    let binary = read_to_string(stp).unwrap();
    for l in binary.lines() {
        let mut ls = l.split(":");
        match ls.next().unwrap() {
            "账号" => {
                account = ls.next().unwrap();
            }
            "密码" => {
                password = ls.next().unwrap();
            }
            "身份" => {
                identity = ls.next().unwrap();
            }
            "运营商" => match ls.next().unwrap() {
                "移动" | "中国移动" => operator = "cmcc",
                "联通" | "中国联通" => operator = "unicom",
                "电信" | "中国电信" => operator = "telecom",
                _ => operator = "none",
            },

            _ => {}
        }
    }

    let cout = Command::new("netsh")
        .args([
            "wlan",
            "connect",
            format!(
                "name=\"{}\"",
                if !(identity == "Student" || identity == "学生" || identity == "student") {
                    TEACHER_WLAN
                } else {
                    STUDENT_WLAN
                }
            )
            .as_str(),
            format!(
                "ssid=\"{}\"",
                if !(identity == "Student" || identity == "学生" || identity == "student") {
                    TEACHER_WLAN
                } else {
                    STUDENT_WLAN
                }
            )
            .as_str(),
        ])
        .output()?;
    let er = String::from_utf8_lossy(&cout.stdout);
    let cr = encoding::all::GBK
        .decode(&cout.stdout, DecoderTrap::Strict)
        .unwrap_or("change false".to_string());
    if er.to_string().contains("successfully") | cr.contains("成功") | er.contains("成功") {
        let qp = get_qp(identity, account, password, operator);
        let mut req = HttpRequest::new();
        req.method = "GET".to_string();
        req.protocol = "HTTP/1.1".to_string();
        req.path = qp;
        req.push_header("host: 172.16.1.38:801");
        let mut buf: Vec<u8>;
        let mut retrytimes = 1;
        loop {
            match req.send() {
                Ok(r) => {
                    buf = r;
                    let rs = String::from_utf8_lossy(&buf);
                    println!("正在连接");
                    std::io::stdout().flush()?;
                    retrytimes = retrytimes + 1;
                    if !rs.contains("RetCode=1&ErrorMsg") {
                        println!("连接成功");
                        std::io::stdout().flush()?;
                        break;
                    }
                    if retrytimes > MAX_RETRY {
                        println!("连接超时，程序退出");
                        std::io::stdout().flush()?;
                        break;
                    }
                    thread::sleep(Duration::from_secs(2));
                    req.path = get_qp(identity, account, password, operator);
                }
                Err(e) => {
                    eprintln!(
                        "已尝试连接{}次\r\n请求失败错误为: {}\r\n尝试再次连接",
                        retrytimes, e
                    );
                    std::io::stdout().flush()?;
                    thread::sleep(Duration::from_secs(2));
                }
            }
        }
        let rs = String::from_utf8_lossy(&buf);
        if rs.contains("RetCode=4&ErrorMsg") {
            thread::sleep(Duration::from_secs(3));
            loop {
                match req.send() {
                    Ok(_r) => {
                        break;
                    }
                    Err(e) => {
                        eprintln!("false retry err:{}", e);
                        std::io::stdout().flush()?;
                        thread::sleep(Duration::from_secs(2));
                    }
                }
            }
        }
    } else {
        println!("未找到校园网")
    }

    Ok(())
}

fn get_qp(identity: &str, account: &str, password: &str, operator: &str) -> String {
    let mut englishr;
    let mut chineser;
    let mut all_ipv4: Vec<String> = Vec::new();
    loop {
        let output = Command::new("ipconfig").output().unwrap();
        chineser = encoding::all::GBK
            .decode(&output.stdout, DecoderTrap::Strict)
            .unwrap_or("False to change".to_string());
        englishr = String::from_utf8_lossy(&output.stdout).to_string();

        if englishr.contains("Wireless LAN adapter WLAN")
            | englishr.contains("无线局域网适配器 WLAN")
        {
            let mut sr = englishr.split("Wireless LAN adapter WLAN");
            for l in sr.nth(1).unwrap().lines() {
                if l.contains("IPv4 地址") | l.contains("IPv4 Address") {
                    all_ipv4.push(l.split(": ").nth(1).unwrap().to_string());
                }
            }
            break;
        }
        if chineser.contains("无线局域网适配器 WLAN") {
            let mut sr = chineser.split("无线局域网适配器 WLAN");
            for l in sr.nth(1).unwrap().lines() {
                if l.contains("IPv4 地址") | l.contains("IPv4 Address") {
                    all_ipv4.push(l.split(": ").nth(1).unwrap().to_string());
                }
            }
            break;
        }
    }

    let addr = all_ipv4[0].clone();
    let qp = if !(identity == "教师" || identity == "Teacher" || identity == "teacher") {
        format!(
        "/eportal/?c=ACSetting&a=Login&loginMethod=1&protocol=http%3A&hostname=172.16.1.38&port=&iTermType=1&wlanuserip={}&wlanacip=172.20.1.1&wlanacname=&redirect=null&session=null&vlanid=0&mac=00-00-00-00-00-00&ip={}&enAdvert=0&jsVersion=2.4.3&DDDDD=%2C0%2C{}%40{}&upass={}&R1=0&R2=0&R3=0&R6=0&para=00&0MKKey=123456&buttonClicked=&redirect_url=&err_flag=&username=&password=&user=&cmd=&Login=&v6ip=",
        addr, addr,account,operator,password,
    )
    } else {
        format!(
        "/eportal/?c=ACSetting&a=Login&loginMethod=1&protocol=http%3A&hostname=172.16.1.38&port=&iTermType=1&wlanuserip={}&wlanacip=172.20.1.1&wlanacname=&redirect=null&session=null&vlanid=0&mac=00-00-00-00-00-00&ip={}&enAdvert=0&jsVersion=2.4.3&DDDDD=%2C0%2C{}&upass={}&R1=0&R2=0&R3=0&R6=0&para=00&0MKKey=123456&buttonClicked=&redirect_url=&err_flag=&username=&password=&user=&cmd=&Login=&v6ip=",
        addr,addr,account,password,
    )
    };
    qp
}

use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
};

use encoding::{DecoderTrap, Encoding};

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

    pub fn send(&mut self) -> std::io::Result<Vec<u8>> {
        let mut host = "localhost:80";
        for h in self.header.iter() {
            if h.contains("host") {
                host = h.split(": ").nth(1).unwrap();
            }
        }
        let mut buf: Vec<u8> = Vec::new();
        let mut ts = TcpStream::connect(host)?;
        ts.write(&self.to_vec_u8())?;
        ts.read_to_end(&mut buf).unwrap();
        Ok(buf)
    }
}

pub trait ToVec {
    fn to_vec_u8(&self) -> Vec<u8>;
}

impl ToVec for String {
    fn to_vec_u8(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl ToVec for &str {
    fn to_vec_u8(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
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
