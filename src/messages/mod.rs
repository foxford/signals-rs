use std::ops::Deref;
use uuid::Uuid;

pub mod agent;
pub mod room;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Envelope {
    pub sub: EnvelopeSubject,
    pub msg: EnvelopeMessage,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EnvelopeSubject {
    pub account_id: Uuid,
    pub agent_id: Uuid,
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
    fn deserialize_envelope() {
        use serde_json;

        let json = r#"{
            "sub": {
                "account_id": "31517b3d-5a14-4a14-a6c5-2ba63f7375d3",
                "agent_id": "85c36f15-5021-4ab8-91a8-0f972cdb6d3a"
            },
            "msg": "ping"
        }"#;

        let envelope: Envelope = serde_json::from_str(json).unwrap();
        let expected = Envelope {
            sub: EnvelopeSubject {
                account_id: Uuid::parse_str("31517b3d-5a14-4a14-a6c5-2ba63f7375d3").unwrap(),
                agent_id: Uuid::parse_str("85c36f15-5021-4ab8-91a8-0f972cdb6d3a").unwrap(),
            },
            msg: EnvelopeMessage("ping".to_string()),
        };

        assert_eq!(envelope, expected);
    }
}
