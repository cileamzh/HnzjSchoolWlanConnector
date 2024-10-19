use crate::{meb::Route, Handler, HttpRequest, HttpResponse, MiddleWare, StaticDir, Ware};

pub struct Router {
    ware_list: Vec<Ware>,
}
impl Router {
    pub fn new() -> Self {
        Router {
            ware_list: Vec::new(),
        }
    }

    pub fn get(&mut self, path: &str, handler: fn(HttpRequest, HttpResponse) -> HttpResponse) {
        self.ware_list.push(Ware::Handler(Handler {
            method: "GET".to_owned(),
            path: path.to_owned(),
            handler: handler,
        }));
    }

    pub fn post(&mut self, path: &str, handler: fn(HttpRequest, HttpResponse) -> HttpResponse) {
        self.ware_list.push(Ware::Handler(Handler {
            method: "POST".to_owned(),
            path: path.to_owned(),
            handler: handler,
        }));
    }

    pub fn push_handler(&mut self, handler: Handler) {
        self.ware_list.push(Ware::Handler(handler));
    }

    pub fn push_midware(&mut self, midware: MiddleWare) {
        self.ware_list.push(Ware::Middleware(midware));
    }

    pub fn midware(
        &mut self,
        midware: fn(HttpRequest, HttpResponse) -> (HttpRequest, HttpResponse),
    ) {
        self.ware_list
            .push(Ware::Middleware(MiddleWare::new(midware)));
    }

    pub fn static_dir(&mut self, path: &str, dir_path: &str) {
        self.ware_list
            .push(Ware::StaticDir(StaticDir::new(path, dir_path)));
    }

    pub fn mount<T: Route>(&mut self, ware: T) {
        ware.mount_self(&mut self.ware_list);
    }
}

impl Route for Router {
    fn mount_self(self, wl: &mut Vec<Ware>) {
        for ware in self.ware_list {
            wl.push(ware);
        }
    }
}
