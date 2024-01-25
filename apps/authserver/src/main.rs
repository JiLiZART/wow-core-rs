use config::ConfigBuilder;


use tokio::net::{TcpListener};






use crate::session::AuthSession;

mod protocol_version;
mod errors;
mod session;

#[tokio::main]
async fn main() {
    let config = ConfigBuilder::new("authserver", "AUTHSERVER");
    let port = config.get_or_default::<String>("port", String::from("3724"));
    let host = config.get_or_default::<String>("host", String::from("0.0.0.0"));

    println!("port {}", port);
    println!("host {}", host);

    let listener = TcpListener::bind(format!("{}:{}", host, port)).await.unwrap();

    println!("Listening {:?}", listener.local_addr());

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let auth = AuthSession::new();

            auth.process(stream).await;
        });
    }
}