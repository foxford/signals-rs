use uuid::Uuid;

use messages::EventKind;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRequest {
    agent_id: Uuid,
    data: UpdateRequestData,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateRequestData {
    online: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEvent {
    payload: UpdateRequest,
}

impl UpdateEvent {
    pub fn agent_id(&self) -> Uuid {
        self.payload.agent_id
    }

    pub fn is_online(&self) -> bool {
        self.payload.data.online
    }
}

impl From<UpdateEvent> for EventKind {
    fn from(event: UpdateEvent) -> Self {
        EventKind::StateUpdate(event)
    }
}
