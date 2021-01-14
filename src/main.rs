fn main() {
    let server = Server::new("127.0.0.1:8080");
}

struct Server {
    addr: String,
}

