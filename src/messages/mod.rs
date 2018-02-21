use serde_json;
use std::ops::Deref;

use errors::*;

pub mod agent;
pub mod room;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    Ping,
    Pong,
}

impl Message {
    pub fn from_envelope(envelope: &Envelope) -> Result<Message> {
        Ok(serde_json::from_str(&envelope.msg.0)?)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Envelope {
    pub sub: EnvelopeSubject,
    pub msg: EnvelopeMessage,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EnvelopeSubject {
    pub account_id: String,
    pub agent_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvelopeMessage(String);

impl Deref for EnvelopeMessage {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_ping_message() {
        let json = r#"{
            "type": "Ping"
        }"#;

        let msg: Message = serde_json::from_str(json).unwrap();
        assert_eq!(msg, Message::Ping);
    }

    #[test]
    fn deserialize_ping_envelope() {
        let json = r#"{
            "sub": {
                "account_id": "31517b3d-5a14-4a14-a6c5-2ba63f7375d3",
                "agent_id": "85c36f15-5021-4ab8-91a8-0f972cdb6d3a"
            },
            "msg": "{\"type\": \"Ping\"}"
        }"#;

        let envelope: Envelope = serde_json::from_str(json).unwrap();
        assert!(Message::from_envelope(&envelope).is_ok());

        let msg = Message::from_envelope(&envelope).unwrap();
        assert_eq!(msg, Message::Ping);

        let json = r#"{
            "sub": {
                "account_id": "31517b3d-5a14-4a14-a6c5-2ba63f7375d3",
                "agent_id": "85c36f15-5021-4ab8-91a8-0f972cdb6d3a"
            },
            "msg": "{\"type\": \"Pin\"}"
        }"#;

        let envelope: Envelope = serde_json::from_str(json).unwrap();
        assert!(Message::from_envelope(&envelope).is_err());
    }
}
