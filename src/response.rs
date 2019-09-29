use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use std::fs::File;

pub enum HttpResponseError {
    Io(io::Error),
}

pub struct HttpResponse {
    version: String,
    code: i32,
    description: String,
    key_values: HashMap<String, String>,
    body: String
}
impl HttpResponse {
    pub fn write_stream<W: Write>(&self, writer: &mut W) -> Result<(), io::Error> {
        let line = format!("HTTP/{} {} {}", self.version, self.code, self.description);
        writeln!(writer, "{}", &line)?;
        for (k, v) in &self.key_values {
            writeln!(writer, "{}: {}", k, v)?;
        }

        // FIXME
//        writeln!(writer, " ")?;
        writeln!(writer)?;

        // FIXME:
        //writeln!(writer, "{}", self.body)?;
        let path = PathBuf::from("/tmp/tmp.txt");
        let mut file = File::create(&path).unwrap();
        write!(file, "{}", self.body).unwrap();
        let mut file = File::open(&path).unwrap();
        io::copy(&mut file, writer).unwrap();

        Ok(())
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
fn contents_len(body: &String) -> u64 {
    let path = PathBuf::from("/tmp/tmp.txt");
    let mut file = File::create(&path).unwrap();
    write!(file, "{}", body).unwrap();

    let file = File::open(&path).unwrap();
    let x= file.metadata().map(|m| m.len()).unwrap_or(0);
    x
}
pub fn ok(body: String, contain_body: bool) -> HttpResponse {

    let len = contents_len(&body);

    let mut key_values = HashMap::new();
    key_values.insert("Content-Type".to_string(), "text/html; charset=UTF-8".to_string());
    key_values.insert("Content-Length".to_string(), format!("{}", len));

    let body = if contain_body { body } else { String::from("") };

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