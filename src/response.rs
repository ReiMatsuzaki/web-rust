use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;

pub struct HttpResponse {
    version: String,
    code: i32,
    description: String,
    key_values: HashMap<String, String>,
    body: String
}
impl HttpResponse {
    pub fn write_stream(&self, stream: &mut TcpStream) {
        let line = format!("HTTP/{} {} {}", self.version, self.code, self.description);
        writeln!(stream, "{}", &line).unwrap();
        for (k, v) in &self.key_values {
            writeln!(stream, "{}: {}", k, v).unwrap();
        }
        writeln!(stream).unwrap();
        writeln!(stream, "{}", self.body).unwrap();
    }
}
fn empty_response(code: i32, description: &str) -> HttpResponse {
    let key_values = HashMap::new();
    let body = String::new();
    HttpResponse{
        version: "1.1".to_string(),
        code,
        description: description.to_string(),
        key_values,
        body
    }
}
pub fn ok(body: String) -> HttpResponse {

    let len = format!("{}", body.len().to_string());

    let mut key_values = HashMap::new();
    key_values.insert("Content-Type".to_string(), "text/html; charset=UTF-8".to_string());
    key_values.insert("Content-Length".to_string(), len);

    return HttpResponse{
        version: "1.1".to_string(),
        code: 200,
        description: "OK".to_string(),
        key_values,
        body};
}
pub fn not_found() -> HttpResponse {
    empty_response(404, "Not Found")
}

pub fn internal_server_error() -> HttpResponse {
    empty_response(500, "Internal Server Error")
}
pub fn not_implemented() -> HttpResponse {
    empty_response(501, "Not Implemented")
}