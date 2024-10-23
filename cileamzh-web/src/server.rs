use crate::{
    meb::{Route, ToVec},
    ware::Ware,
    HttpRequest, HttpResponse,
};
use std::{
    env::current_dir,
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
};
/// HttpServer represent a server for the communication with Http protocal
///
/// # Examples
///
/// ```
/// let mut server=HttpServer::new();
/// ```
pub struct HttpServer {
    lst: Vec<TcpListener>,
    warelist: Vec<Ware>,
}
impl HttpServer {
    ///Create a httpserver
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut server=HttpServer::new();
    /// ```
    pub fn new() -> Self {
        HttpServer {
            lst: Vec::new(),
            warelist: Vec::new(),
        }
    }
    ///Listen a host and start your server.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let server=HttpServer::new();
    /// server.listen("localhost",8080)?;
    /// ```
    ///
    pub fn listen(&mut self, mut host: &str, port: u64) -> std::io::Result<()> {
        if host == "localhost" {
            host = "127.0.0.1"
        }
        let lst = TcpListener::bind(format!("{}:{}", host, port))?;
        self.lst.push(lst);
        for stream in self.lst[0].incoming() {
            let _stream = stream?;
            let _warelist = &self.warelist;
        }
        Ok(())
    }
    ///Quickly listen localhost
    ///
    /// #Example
    /// ```rust
    /// server.localhost(8080)?;
    /// ```
    pub fn localhost(&mut self, port: u64) -> std::io::Result<()> {
        self.listen("localhost", port)?;
        Ok(())
    }

    ///Mount you middleware and router
    ///
    /// #Examples
    ///
    /// ```rust
    /// server.mount(your_midware);
    /// server.mount(your_router);
    /// ```
    pub fn mount<T: Route>(&mut self, ware: T) {
        ware.mount_self(&mut self.warelist);
    }
    ///Set a dir as static dir.
    ///
    /// #Exmaple
    ///
    /// ```rust
    /// server.set_static_dir(route_path,dir_path);
    /// ```
    pub fn set_static_dir(_route_path: &str, dir_path: &str) {
        if Path::new(dir_path).is_dir() {
        } else {
        }
    }

    //to handle tcpstream as http
}
fn contains_array(outer: Vec<u8>, inner: &[u8]) -> bool {
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

pub fn _handle_stream(warelist: &Vec<Ware>, mut stream: &TcpStream) -> std::io::Result<()> {
    let parten = "\r\n\r\n".as_bytes();
    let mut buffer = [0; 512];
    let mut binary_http: Vec<u8> = Vec::new();
    loop {
        let len = stream.read(&mut buffer)?;
        if len > 0 {
            binary_http.append(&mut buffer.to_vec());
        }
        if len < buffer.len() {
            break;
        }
    }
    if contains_array(binary_http.clone(), parten) {
        let req: HttpRequest = HttpRequest::from(binary_http);
        let res: HttpResponse = HttpResponse::new();
        let formot = through_ware(warelist, req, res).to_vec_u8();
        stream.write(&formot)?;
    }
    Ok(())
}

pub fn through_ware(
    warelist: &Vec<Ware>,
    mut req: HttpRequest,
    mut res: HttpResponse,
) -> HttpResponse {
    let mut ware_index: usize = 0;
    loop {
        if ware_index >= warelist.len() {
            break;
        }
        match &warelist[ware_index] {
            Ware::Handler(handler) => {
                let (m, p) = (&req.method, &req.path);
                if (m, p) == (&handler.method, &handler.path) {
                    res = (handler.handler)(req, res);
                    break;
                }
            }
            Ware::Middleware(middleware) => {
                (req, res) = (middleware.route)(req, res);
            }
            Ware::StaticDir(staticdir) => {
                println!("{}", String::from_utf8_lossy(&req.to_vec_u8()));
                if staticdir.path == req.path && !staticdir.index.is_empty() {
                    let req_path = format!(
                        "{}{}{}",
                        current_dir().unwrap().to_string_lossy(),
                        staticdir.dir_path,
                        staticdir.index,
                    );
                    res.binary = fs::read(req_path).unwrap_or("Don't find Index".to_vec_u8());
                    break;
                }
                if !(req.path.len() > staticdir.path.len()
                    && staticdir.path == req.path[..staticdir.path.len()])
                {
                    continue;
                }
                let req_path = format!(
                    "{}{}{}",
                    current_dir().unwrap().to_string_lossy(),
                    staticdir.dir_path,
                    req.path.replace(&staticdir.path, ""),
                );
                let f_path = Path::new(&req_path);
                if !(f_path.exists() && f_path.is_file()) {
                    res.binary = "Can't Get".to_vec_u8();
                    break;
                }
                res.set_status("200 OK");
                res.binary = fs::read(f_path).unwrap_or("Fail to read file".to_vec_u8());
                break;
            }
        }
        ware_index = ware_index + 1;
    }
    res.set_header(&format!(
        "Content-Length: {}",
        (res.binary.len() + res.body.len()).to_string()
    ));
    res
}
