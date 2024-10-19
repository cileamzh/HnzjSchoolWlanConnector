use crate::{meb::Route, HttpRequest, HttpResponse};

///Router is used to route the communication
pub struct Handler {
    pub method: String,
    pub path: String,
    pub handler: fn(HttpRequest, HttpResponse) -> HttpResponse,
}

impl Handler {
    pub fn new(
        method: &str,
        path: &str,
        handler: fn(HttpRequest, HttpResponse) -> HttpResponse,
    ) -> Handler {
        Handler {
            method: method.to_owned(),
            path: path.to_owned(),
            handler,
        }
    }
}

impl Route for Handler {
    fn mount_self(self, wl: &mut Vec<Ware>) {
        wl.push(Ware::Handler(self));
    }
}

pub struct MiddleWare {
    pub route: fn(HttpRequest, HttpResponse) -> (HttpRequest, HttpResponse),
}

impl MiddleWare {
    pub fn new(route: fn(HttpRequest, HttpResponse) -> (HttpRequest, HttpResponse)) -> Self {
        MiddleWare { route: route }
    }
}

impl Route for MiddleWare {
    fn mount_self(self, wl: &mut Vec<Ware>) {
        wl.push(Ware::Middleware(self));
    }
}

pub struct StaticDir {
    pub path: String,
    pub dir_path: String,
    pub index: String,
}

impl StaticDir {
    pub fn new(path: &str, dir_path: &str) -> Self {
        StaticDir {
            path: path.to_owned(),
            dir_path: dir_path.to_owned(),
            index: String::new(),
        }
    }

    pub fn index(&mut self, file: &str) {
        self.index = file.to_owned();
    }
}

impl Route for StaticDir {
    fn mount_self(self, wl: &mut Vec<Ware>) {
        wl.push(Ware::StaticDir(self));
    }
}

/// emum Ware contains two members,Router and MiddleWare
///
///  
pub enum Ware {
    Handler(Handler),
    Middleware(MiddleWare),
    StaticDir(StaticDir),
}
