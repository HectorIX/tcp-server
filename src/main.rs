extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;


use tokio_proto::TcpServer;

mod service_state;
mod encryptor_services_list;
mod parser;
mod user;
mod codec;
mod file_io;
mod sign_up;
mod sign_in;
mod upload;
mod download;



fn main() {
    // Specify the localhost address
    let addr = "127.0.0.1:4000".parse().unwrap();

    // The builder requires a protocol and an address
    let server = TcpServer::new(codec::LineProto, addr);
    println!("Listening on http://{}", addr);
    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(|| Ok(codec::Encryptor));

}
