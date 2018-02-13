use serde_json;

use errors::*;
use messages::agent::*;
use messages::room::*;

mod agent;
mod room;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    Ping,
    Pong,

    RoomsCreateRequest(RoomsCreateRequest),
    RoomsCreateResponse(RoomsCreateResponse),
    RoomsReadRequest(RoomsReadRequest),
    RoomsReadResponse(RoomsReadResponse),
    RoomsUpdateRequest(RoomsUpdateRequest),
    RoomsUpdateResponse(RoomsUpdateResponse),
    RoomsDeleteRequest(RoomsDeleteRequest),
    RoomsDeleteResponse(RoomsDeleteResponse),
    RoomsListRequest(RoomsListRequest),
    RoomsListResponse(RoomsListResponse),

    AgentsCreateRequest(AgentsCreateRequest),
    AgentsCreateResponse(AgentsCreateResponse),
    AgentsReadRequest(AgentsReadRequest),
    AgentsReadResponse(AgentsReadResponse),
    AgentsUpdateRequest(AgentsUpdateRequest),
    AgentsUpdateResponse(AgentsUpdateResponse),
    AgentsDeleteRequest(AgentsDeleteRequest),
    AgentsDeleteResponse(AgentsDeleteResponse),
    AgentsListRequest(AgentsListRequest),
    AgentsListResponse(AgentsListResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Envelope {
    sub: EnvelopeSubject,
    msg: EnvelopeMessage,
}

impl Envelope {
    pub fn message(&self) -> Result<Message> {
        Ok(serde_json::from_str(&self.msg.0)?)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct EnvelopeSubject {
    account_id: String,
    agent_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct EnvelopeMessage(String);

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
        assert!(envelope.message().is_ok());

        let msg = envelope.message().unwrap();
        assert_eq!(msg, Message::Ping);

        let json = r#"{
            "sub": {
                "account_id": "31517b3d-5a14-4a14-a6c5-2ba63f7375d3",
                "agent_id": "85c36f15-5021-4ab8-91a8-0f972cdb6d3a"
            },
            "msg": "{\"type\": \"Pin\"}"
        }"#;

        let envelope: Envelope = serde_json::from_str(json).unwrap();
        assert!(envelope.message().is_err());
    }
}
