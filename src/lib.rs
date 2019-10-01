pub mod request;
pub mod response;
pub mod dispatcher;
pub mod ssr;
pub mod web_server;

#[cfg(test)]
mod tests {
    use base64::{encode, decode};

    #[test]
    fn check_base64_encode() {
        let name_pass = "aa:bb";
        let coded = "YWE6YmI=";
        let res = encode(name_pass);
        assert_eq!(coded, res);

        let res = decode(coded).unwrap();//map(|s| s as str);
        let res: String = res.iter().map(|&s| s as char).collect();
        assert_eq!(res, name_pass);
    }
}

