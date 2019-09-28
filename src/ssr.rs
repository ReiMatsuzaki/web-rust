use crate::response::{self, HttpResponse};

pub fn dispatch_ssr(path: &String) -> HttpResponse {
    match path.as_str() {
        "page1" => ssr_page1(path),
        _ => response::not_found(),
    }
}

fn ssr_page1(path: &String) -> HttpResponse {
    let body = format!("<p>this is SSR sample</p><p>path: {}</p>", path);
    response::ok(body, true)
}