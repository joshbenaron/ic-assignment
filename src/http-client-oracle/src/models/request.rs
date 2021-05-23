#[derive(Clone)]
pub struct Request {
    pub method: HttpMethod,
    pub url: String
}
#[derive(Clone, Copy)]
pub enum HttpMethod { GET, POST, PUT }

impl HttpMethod {
    pub fn as_str(&self) -> &str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT"
        }
    }
}

impl Into<awc::http::Method> for HttpMethod {
    fn into(self) -> awc::http::Method {
        match self {
            HttpMethod::GET => awc::http::Method::GET,
            HttpMethod::POST => awc::http::Method::POST,
            HttpMethod::PUT => awc::http::Method::PUT
        }
    }
}

impl From<&str> for HttpMethod {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            s => panic!("Received invalid method: {}", s)
        }
    }
}