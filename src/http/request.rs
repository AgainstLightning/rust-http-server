use super::Method;

pub struct Request {
    path: String,
    query: Option<String>,
    method: Method,
}
