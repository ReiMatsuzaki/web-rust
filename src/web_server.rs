use crate::dispatcher::Dispatcher;
use crate::request;
use crate::response;
use std::collections::HashMap;

pub struct WebServerStatus {
    key_value: HashMap<String, String>,
}
impl WebServerStatus {
    pub fn new() -> WebServerStatus {
        let key_value: HashMap<String, String> = HashMap::new();
        WebServerStatus{key_value}
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
    pub fn response(&mut self, req: &request::HttpRequest) -> response::HttpResponse {
        self.dispatcher.dispatch(req, &mut self.status)
    }
}