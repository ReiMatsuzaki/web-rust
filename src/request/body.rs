use std::collections::HashMap;

pub struct Body {
    value: HashMap<String, String>
}

impl Body {
    pub fn new() -> Body {
        let value: HashMap<String, String> = HashMap::new();
        Body { value }
    }
    pub fn to_string(&self) -> String {
        let kvs: Vec<String> = self.value.iter().map(
            |(k, v)| format!("{}={}", k, v)).collect();
        kvs.join("&")
    }
    pub fn insert(&mut self, k: String, v: String) {
        self.value.insert(k, v);
    }
    pub fn get(&self, k: &str) -> Option<&String> {
        self.value.get(k)
    }
}