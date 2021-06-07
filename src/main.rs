mod server;
use server::Server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod http {
    pub mod request {
        use super::method::Method;

        struct Request {
            path: String,
            query: Option<String>,
            method: Method,
        }
    }

    mod method {
        pub enum Method {
            GET,
            DELETE,
            POST,
            PUT,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH,
        }
    }
}
