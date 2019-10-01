pub struct LeadLine {
    pub method: String,
    pub path: String,
    pub version: String,
}
impl LeadLine {
    pub fn get(path: String) -> LeadLine {
        LeadLine{
            method: "GET".to_string(),
            path,
            version: "1.1".to_string()}
    }
    pub fn post(path: String) -> LeadLine {
        LeadLine{
            method: "POST".to_string(),
            path,
            version: "1.1".to_string()}
    }
    pub fn to_string(&self) -> String {
        format!("{} {} HTTP/{}\n", self.method, self.path, self.version)
    }
}