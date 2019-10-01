use std::collections::HashMap;
use crate::request::http_request::HttpRequest;
use crate::response::http_response::HttpResponse;
use crate::web_server::dispatcher::Dispatcher;

pub struct WebServerStatus {
    key_value: HashMap<String, String>,
}
impl WebServerStatus {
    pub fn new() -> WebServerStatus {
        let key_value: HashMap<String, String> = HashMap::new();
        WebServerStatus{key_value}
    }
    pub fn insert(&mut self, k: String, v: String) {
        self.key_value.insert(k, v);
    }
}

pub struct WebServer{
    dispatcher: Dispatcher,
    pub status: WebServerStatus,
}
impl WebServer {
    pub fn new() -> WebServer {
        let dispatcher = Dispatcher{};
        let status = WebServerStatus::new();
        WebServer{dispatcher, status}
    }
    pub fn response(&mut self, req: &HttpRequest) -> HttpResponse {
        self.dispatcher.dispatch(req, &mut self.status)
    }
}