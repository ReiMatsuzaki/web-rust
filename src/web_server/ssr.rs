use crate::request;
use log::{info, error};
use std::fmt;
use base64::{decode, DecodeError};
use crate::request::HttpRequest;
use crate::response::HttpResponse;
use crate::web_server::WebServerStatus;

#[derive(Debug)]
pub enum SsrError {
    KeyNotExists { key: String },
    InvalidLine { line: String },
    DecodeError(DecodeError),
}

impl fmt::Display for SsrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SsrError::KeyNotExists { ref key } => write!(f, "key: {}", key),
            SsrError::InvalidLine { ref line } => write!(f, "line: {}", line),
            SsrError::DecodeError(e) => write!(f, "{}", e),
        }
    }
}

pub fn dispatch_ssr(path: &String,
                    req: &HttpRequest,
                    status: &mut WebServerStatus) -> HttpResponse {
    info!("dispatch_str begin");
    let result = match path.as_str() {
        "page1" => ssr_page1(path),
        "31_002" => ssr_31_002(),
        "31_003" => ssr_31_003(&req.body),
        "31_004" => ssr_31_004(&req.body),
        "31_010" => ssr_31_010(req),
        "31_011" => ssr_31_011(req, status),
        _ => Ok(HttpResponse::not_found()),
    };
    match result {
        Ok(x) => x,
        Err(e) => {
            error!("{}", e);
            HttpResponse::internal_server_error()
        }
    }
}

fn ssr_page1(path: &String) -> Result<HttpResponse, SsrError> {
    let body = format!("<p>this is SSR sample</p><p>path: {}</p>", path);
    Ok(HttpResponse::ok(body, true))
}

fn ssr_31_002() -> Result<HttpResponse, SsrError> {
    info!("ssr_31_002 begin");
    let body = r#"<html>
    <head><title>personal information input</title></head>
    <body>
        <form action="31_003" method="POST">
        name<input type="text" name="name"><br>
        mail<input type="text" name="mail"><br>
        gender <input type="radio" name="gender" value="woman">woman
        <input type="radio" name="gender" value="man">man<br>
        <input type="submit" value="check">
        </form>
    </body>
    </html>
    "#;
    let body = format!("{}", body);
    Ok(HttpResponse::ok(body, true))
}

fn ssr_31_003(req_body: &request::Body) -> Result<HttpResponse, SsrError> {
    info!("ssr_31_003 begin");
    let name = match req_body.get("name") {
        Some(x) => x,
        None => return Err(SsrError::KeyNotExists { key: "name".to_string() }),
    };
    let mail = match req_body.get("mail") {
        Some(x) => x,
        None => return Err(SsrError::KeyNotExists { key: "mail".to_string() }),
    };
    let gender = match req_body.get("gender") {
        Some(x) => x,
        None => return Err(SsrError::KeyNotExists { key: "gender".to_string() }),
    };

    let body = format!(r#"<html>
    <head><title>check</title></head>
    <body>
        name: {}<br>
        mail: {}<br>
        gender: {}<br>
        <form action="31_004" method="POST">
            <input type="hidden" name="name" value="{}"><br>
            <input type="hidden" name="mail" value="{}"><br>
            <input type="hidden" name="gender" value="{}">
            <input type="submit" value="check">
        </form>
    </body>
    </html>
    "#, name, mail, gender, name, mail, gender);
    Ok(HttpResponse::ok(body, true))
}

fn ssr_31_004(req_body: &request::Body) -> Result<HttpResponse, SsrError> {
    info!("ssr_31_004 begin");
    let name = match req_body.get("name") {
        Some(x) => x,
        None => return Err(SsrError::KeyNotExists { key: "name".to_string() }),
    };
    let mail = match req_body.get("mail") {
        Some(x) => x,
        None => return Err(SsrError::KeyNotExists { key: "mail".to_string() }),
    };
    let gender = match req_body.get("gender") {
        Some(x) => x,
        None => return Err(SsrError::KeyNotExists { key: "gender".to_string() }),
    };

    let body = format!(r#"<html>
    <head><title>check</title></head>
    <body>
        name: {}<br>
        mail: {}<br>
        gender: {}<br>
        registered!!
    </body>
    </html>
    "#, name, mail, gender);
    Ok(HttpResponse::ok(body, true))
}

fn ssr_31_010(req: &HttpRequest) -> Result<HttpResponse, SsrError> {
    let header = &req.header;
    let line = header.get("Authorization");
    match line {
        None => Ok(HttpResponse::unauthorized()),
        Some(line) => {
            let mut xs = line.trim().split_whitespace();
            let x0 = xs.next().unwrap().trim();
            if x0 != "Basic" {
                Err(SsrError::InvalidLine { line: x0.to_string() })
            } else {
                let coded = xs.next().unwrap();
                match decode(coded) {
                    Err(e) => Err(SsrError::DecodeError(e)),
                    Ok(decoded) => {
                        let decoded: String = decoded.iter().map(|&s| s as char).collect();
                        if decoded == "aa:bb" {
                            Ok(ssr_31_010_page())
                        } else {
                            Ok(HttpResponse::unauthorized())
                        }
                    }
                }
            }
        }
    }
}

fn ssr_31_010_page() -> HttpResponse {
    let body = format!("<html>
    <head><title>check</title></head>
    <body>
        name: aa<br>
        pass: bb<br>
    </body>
    </html>
    ");
    HttpResponse::ok(body, true)
}

fn ssr_31_011(_: &HttpRequest, _: &mut WebServerStatus) -> Result<HttpResponse, SsrError> {
    Ok(HttpResponse::not_found())
}

//#[test]
//fn ssr_test() {
//    use base64::encode;
//
//    fn get_req(name_pass: &str) -> HttpRequest {
//        let value = format!("method {}", encode(name_pass));
//
//        let lead_line = request::lead_line::LeadLine::get("path".to_string());
//        let mut header = request::header::Header::new();
//        header.insert("Authorization".to_string(), value);
//        let header = header;
//
//        let body = request::body::Body::new();
//
//        HttpRequest { lead_line, header, body }
//    }
//
//    let req = get_req("aa:bb");
//    let rep = ssr_31_010(&req);
//    let res = rep.map(|x| x.code).unwrap();
//    assert_eq!(res, 200);
//
//    let req = get_req("aa:bc");
//    let rep = ssr_31_010(&req);
//    let res = rep.map(|x| x.code).unwrap();
//    assert_eq!(res, 401);
//}