use jsonrpc_core::{self, Params, Version};
use serde::ser::Serialize;
use serde_json;
use std::ops::Deref;
use uuid::Uuid;

pub mod agent;
pub mod room;
pub mod subscription;
pub mod track;
pub mod query_parameters;
pub mod webrtc;

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

#[derive(Debug)]
pub enum Notification {
    Event(EventKind),
    Method(Method),
}

impl From<Notification> for jsonrpc_core::Notification {
    fn from(notification: Notification) -> Self {
        match notification {
            Notification::Event(e) => e.into(),
            Notification::Method(m) => m.body,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum EventKind {
    #[serde(rename = "agent.create")]
    AgentCreate(agent::CreateEvent),
    #[serde(rename = "agent.delete")]
    AgentDelete(agent::DeleteEvent),
    #[serde(rename = "track.create")]
    TrackCreate(track::CreateEvent),
    #[serde(rename = "track.update")]
    TrackUpdate(track::UpdateEvent),
    #[serde(rename = "track.delete")]
    TrackDelete(track::DeleteEvent),
}

impl From<EventKind> for Notification {
    fn from(kind: EventKind) -> Self {
        Notification::Event(kind)
    }
}

impl From<EventKind> for jsonrpc_core::Notification {
    fn from(event: EventKind) -> Self {
        let params = serde_json::to_value(event)
            .ok()
            .map(|value| Params::Array(vec![value]));

        jsonrpc_core::Notification {
            jsonrpc: Some(Version::V2),
            method: "event".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Event<T: Serialize> {
    #[serde(skip_serializing)]
    pub room_id: Uuid,
    payload: T,
}

impl<T: Serialize> Event<T> {
    pub fn new(room_id: Uuid, payload: T) -> Event<T> {
        Event { room_id, payload }
    }
}

#[derive(Debug)]
pub struct Method {
    pub agent_id: Uuid,
    pub body: jsonrpc_core::Notification,
}

impl From<Method> for Notification {
    fn from(method: Method) -> Self {
        Notification::Method(method)
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
