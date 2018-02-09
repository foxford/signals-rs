use serde_json;

use errors::*;
use models;

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
}

// RoomsCreate

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RoomsCreateRequest {
    pub payload: RoomsCreateRequestPayload,
    cid: String,
}

type RoomsCreateRequestPayload = models::NewRoom;

impl RoomsCreateRequest {
    pub fn build_response(self, room: &models::Room) -> RoomsCreateResponse {
        RoomsCreateResponse {
            payload: RoomsCreateResponsePayload {
                id: room.id.to_string(),
                data: RoomsCreateResponseData {
                    label: Some(room.label.clone()), // FIXME: avoid clone()
                },
            },
            cid: self.cid,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RoomsCreateResponse {
    payload: RoomsCreateResponsePayload,
    cid: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct RoomsCreateResponsePayload {
    id: String,
    data: RoomsCreateResponseData,
}

type RoomsCreateResponseData = RoomsCreateRequestPayload;

// RoomsCreate

// RoomsRead

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RoomsReadRequest {
    cid: String,
}

impl RoomsReadRequest {
    pub fn build_response(self, room: &models::Room) -> RoomsReadResponse {
        RoomsReadResponse {
            payload: RoomsReadResponsePayload {
                id: room.id.to_string(),
                data: RoomsReadResponseData {
                    label: Some(room.label.clone()), // FIXME: avoid clone()
                },
            },
            cid: self.cid,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RoomsReadResponse {
    payload: RoomsReadResponsePayload,
    cid: String,
}

type RoomsReadResponsePayload = RoomsCreateResponsePayload;
type RoomsReadResponseData = RoomsCreateResponseData;

// RoomsRead

// RoomsUpdate

pub type RoomsUpdateRequest = RoomsCreateRequest;
pub type RoomsUpdateResponse = RoomsCreateResponse;

// RoomsUpdate

// RoomsDelete

pub type RoomsDeleteRequest = RoomsReadRequest;
pub type RoomsDeleteResponse = RoomsReadResponse;

// RoomsDelete

// RoomsList

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RoomsListRequest {
    cid: String,
}

impl RoomsListRequest {
    pub fn build_response(self, rooms: &Vec<models::Room>) -> RoomsListResponse {
        let payload: Vec<RoomsListResponsePayload> = rooms
            .iter()
            .map(|room| {
                RoomsListResponsePayload {
                    id: room.id.to_string(),
                    data: RoomsListResponseData {
                        label: Some(room.label.clone()), // FIXME: avoid clone()
                    },
                }
            })
            .collect();

        RoomsListResponse {
            payload: payload,
            cid: self.cid,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RoomsListResponse {
    payload: Vec<RoomsListResponsePayload>,
    cid: String,
}

type RoomsListResponsePayload = RoomsReadResponsePayload;
type RoomsListResponseData = RoomsReadResponseData;

// RoomsList

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
    fn de_ping() {
        let json = r#"{
            "type": "Ping"
        }"#;

        let msg: Message = serde_json::from_str(json).unwrap();
        assert_eq!(msg, Message::Ping);
    }

    #[test]
    fn de_ping_envelope() {
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
    }

    #[test]
    fn de_ping_envelope_err() {
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
