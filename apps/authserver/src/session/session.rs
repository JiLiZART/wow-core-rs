
use tokio::net::{TcpStream};
use wow_srp::normalized_string::NormalizedString;
use wow_srp::server::{SrpProof, SrpVerifier};
use std::collections::HashMap;
use std::os::unix::process;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener};
use wow_login_messages::all::{
    CMD_AUTH_LOGON_CHALLENGE_Client, 
    CMD_AUTH_RECONNECT_CHALLENGE_Client
};
use wow_login_messages::errors::ExpectedOpcodeError;
use wow_login_messages::helper::{
    tokio_expect_client_message, 
    tokio_read_initial_message, 
    InitialMessage,
};
use wow_login_messages::ServerMessage;
use wow_srp::{PublicKey, GENERATOR, LARGE_SAFE_PRIME_LITTLE_ENDIAN};

pub(crate) struct AuthSession {

}

fn get_proof(username: &str) -> SrpProof {
    let username = NormalizedString::new(username.to_string()).unwrap();
    let password = NormalizedString::new(username.to_string()).unwrap();

    SrpVerifier::from_username_and_password(username, password).into_proof()
}

impl AuthSession {

    pub fn new() -> Self {
        AuthSession {}
    }

    pub async fn process(&self, mut stream: TcpStream) {
        let opcode = tokio_read_initial_message(&mut stream).await;

        let opcode = match opcode {
            Ok(o) => o,
            Err(e) => {
                match e {
                    ExpectedOpcodeError::Opcode(o) => {
                        println!("invalid opcode {o}")
                    }
                    ExpectedOpcodeError::Parse(e) => {
                        println!("parse error {e:#?}")
                    }
                }
                return;
            }
        };

        // Wrath can only use protocol version eight
        match opcode {
            InitialMessage::Logon(l) => {
                if let 0x8 = l.protocol_version {
                    &self.login_version_8(stream, l).await;
                }
            }
            InitialMessage::Reconnect(r) => {
                if let 0x8 = r.protocol_version {
                    println!("protocol version eight reconnect");
                }
            }
        }
    }

    pub async fn login_version_8(
        &self,
        mut stream: TcpStream,
        l: CMD_AUTH_LOGON_CHALLENGE_Client,
    ) {
        use wow_login_messages::version_8::*;

        println!("Login version: {}", l.protocol_version);

        let p = get_proof(&l.account_name);
        let username = l.account_name;

        CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                server_public_key: *p.server_public_key(),
                generator: vec![GENERATOR],
                large_safe_prime: LARGE_SAFE_PRIME_LITTLE_ENDIAN.into(),
                salt: *p.salt(),
                crc_salt: [0; 16],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty(),
            },
        }
        .tokio_write(&mut stream)
        .await
        .unwrap();

        println!("Sent Logon Challenge");

        let l = tokio_expect_client_message::<CMD_AUTH_LOGON_PROOF_Client, _>(&mut stream)
            .await
            .unwrap();

        let (p, server_proof) = p
            .into_server(
                PublicKey::from_le_bytes(l.client_public_key).unwrap(),
                l.client_proof,
            )
            .unwrap();

        CMD_AUTH_LOGON_PROOF_Server {
            result: CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                account_flag: AccountFlag::empty(),
                server_proof,
                hardware_survey_id: 0,
                unknown_flags: 0,
            },
        }
        .tokio_write(&mut stream)
        .await
        .unwrap();
        
        println!("Sent Logon Proof");
    }
}