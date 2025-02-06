use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    method: String,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    body: String,

}

impl HttpRequest {
    pub fn new (method: String, path: String, version: String, headers: HashMap<String, String>, body: String) -> HttpRequest {
        HttpRequest {
            method: method,
            path: path,
            version: version,
            headers: headers,
            body: body,
        }
    }
}