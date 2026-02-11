wit_bindgen::generate!({
    world: "wasmcloud:messaging/messaging-consumer",
});

use exports::wasmcloud::messaging::consumer::{Guest, BrokerMessage};

struct StreamReceiver;

export!(StreamReceiver);

impl Guest for StreamReceiver {
    fn handle_message(msg: BrokerMessage) -> Result<(), String> {
        // Convert the message body to a string
        let body = String::from_utf8(msg.body)
            .map_err(|e| format!("Failed to parse message body as UTF-8: {}", e))?;

        // Log the received message (in a real component, you might process it differently)
        eprintln!("Received message on subject '{}': {}", msg.subject, body);

        // Optionally handle reply-to if present
        if let Some(reply_to) = msg.reply_to {
            eprintln!("Message expects reply to: {}", reply_to);
        }

        Ok(())
    }
}
