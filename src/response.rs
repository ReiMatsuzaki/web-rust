use std::collections::HashMap;
use std::io::{self, Write};

pub enum HttpResponseError {
    Io(io::Error),
}

pub struct HttpResponse {
    version: String,
    pub code: i32,
    description: String,
    key_values: HashMap<String, String>,
    body: String
}
impl HttpResponse {
    fn default_http_version() -> String {
        "1.1".to_string()
    }
    pub fn write_stream<W: Write>(&self, writer: &mut W) -> Result<(), io::Error> {
        let line = format!("HTTP/{} {} {}", self.version, self.code, self.description);
        writeln!(writer, "{}", &line)?;
        for (k, v) in &self.key_values {
            writeln!(writer, "{}: {}", k, v)?;
        }
        writeln!(writer)?;
        writeln!(writer, "{}", self.body)?;

        Ok(())
    }

}
fn empty_response(code: i32, description: &str) -> HttpResponse {
    let key_values = HashMap::new();
    let body = String::new();
    HttpResponse{
        version: HttpResponse::default_http_version(),
        code,
        description: description.to_string(),
        key_values,
        body
    }
}
fn contents_len(body: &String) -> String {
    let x=  body.len();
    format!("{}", x)
}
pub fn ok(body: String, contain_body: bool) -> HttpResponse {

    let len = contents_len(&body);

    let mut key_values = HashMap::new();
    key_values.insert("Content-Type".to_string(), "text/html; charset=UTF-8".to_string());
    key_values.insert("Content-Length".to_string(), len);

    let body = if contain_body { body } else { String::from("") };

    return HttpResponse{
        version: HttpResponse::default_http_version(),
        code: 200,
        description: "OK".to_string(),
        key_values,
        body};
}

pub fn unauthorized() -> HttpResponse {
    let mut key_values = HashMap::new();
    key_values.insert("WWW-Authenticate".to_string(),
                  "Basic realm=\"Basic Authentication Sample\"".to_string());
    let body = "".to_string();
    HttpResponse {
        version: HttpResponse::default_http_version(),
        code: 401,
        description: "Unauthorized".to_string(),
        key_values,
        body}
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