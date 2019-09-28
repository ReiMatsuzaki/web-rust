use crate::response::{self, HttpResponse};
use crate::request;

pub fn dispatch_ssr(path: &String, req_body: &request::Body) -> HttpResponse {
    match path.as_str() {
        "page1" => ssr_page1(path),
        "31_002" => ssr_31_002(),
        "31_003" => ssr_31_003(req_body),
        _ => response::not_found(),
    }
}
fn ssr_31_002() -> HttpResponse {
    let body = r#"<html>
    <head><title>personal information input</title></head>
    <body>
        <form action="31_003" method="POST">
        name<input type="text" name="name"><br>
        mail<input type="text" name="mail"><br>
        gender <input type="radio" name="gender" value="femail">femail
        <input type="radio" name="gender" value="mail">mail<br>
        <input type="submit" value="check">
        </form>
    </body>
    </html>
    "#;
    let body = format!("{}", body);
    response::ok(body, true)
}
fn ssr_31_003(req_body: &request::Body) -> HttpResponse {
    let name = req_body.get("name");
    let mail = req_body.get("mail");
    let gender = req_body.get("gender");
    let body = format!(r#"<html>
    <head><title>check</title></head>
    <body>
        <form action="31_004" method="POST">
        name: {}<br>
        mail: {}<br>
        gender: {}<br>
        <input type="submit" value="check">
        </form>
    </body>
    </html>
    "#, name, mail, gender);
    let body = format!("{}", body);
    response::ok(body, true)
}
fn ssr_page1(path: &String) -> HttpResponse {
    let body = format!("<p>this is SSR sample</p><p>path: {}</p>", path);
    response::ok(body, true)
}