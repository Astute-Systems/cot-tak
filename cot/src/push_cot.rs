
use crate::CotConfig;
use crate::message::compose_message;
use crate::SocketConnection;

pub fn push_cot(cot_config: CotConfig, client_address: &str, client_port: u16) {
    // Validate config
    if !cot_config.attachment_paths.is_empty() {
        panic!("Pushing cursor on target messages with attachments requires a file server to serve attachments. Please use `CotServer.push_cot` instead");
    }

    // Compose message
    let message = compose_message(&cot_config, None, None, None).unwrap();

    // Send message
    let mut socket_connection = SocketConnection::new(client_address, client_port);
    socket_connection.send(&message);
}
