use config::ConfigBuilder;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener};
use wow_login_messages::all::{
    CMD_AUTH_LOGON_CHALLENGE_Client, CMD_AUTH_RECONNECT_CHALLENGE_Client
};
use wow_login_messages::errors::ExpectedOpcodeError;
use wow_login_messages::helper::{
    tokio_expect_client_message, tokio_read_initial_message, InitialMessage,
};
use wow_login_messages::ServerMessage;

use crate::protocol_version::ProtocolVersion;
use crate::session::AuthSession;

mod protocol_version;
mod errors;
mod session;

#[tokio::main]
async fn main() {
    let config = ConfigBuilder::new("authserver", "AUTHSERVER");
    let port = config.get_or_default::<String>("port", String::from("3724"));
    let host = config.get_or_default::<String>("host", String::from("0.0.0.0"));

    let listener = TcpListener::bind(host + port.as_str()).await.unwrap();

    println!("Listening {:?}", listener.local_addr());

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let auth = AuthSession::new();

            auth.process(stream).await;
        });
    }
}