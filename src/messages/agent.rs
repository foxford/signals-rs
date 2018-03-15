use chrono::NaiveDateTime;
use uuid::Uuid;

use messages::query_parameters::QueryParameters;
use messages::{Event, EventKind};
use models;

// Create

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct CreateResponse {
    id: Uuid,
}

impl CreateResponse {
    pub fn new(agent: &models::Agent) -> CreateResponse {
        CreateResponse { id: agent.id }
    }
}

// Create

// Read

#[derive(Debug, Deserialize)]
pub struct ReadRequest {
    pub room_id: Uuid,
    pub id: Uuid,
}

#[derive(Clone, Debug, Serialize)]
pub struct ReadResponse {
    id: Uuid,
    data: ReadResponseData,
}

impl ReadResponse {
    pub fn new(agent: &models::RoomAgent) -> ReadResponse {
        ReadResponse {
            id: agent.agent_id,
            data: ReadResponseData {
                label: agent.label.clone(),
                created_at: agent.created_at,
            },
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct ReadResponseData {
    label: String,
    created_at: NaiveDateTime,
}

// Read

// Update

pub type UpdateRequest = JoinRequest;
pub type UpdateResponse = JoinResponse;

// Update

// Delete

pub type DeleteRequest = CreateRequest;
pub type DeleteResponse = CreateResponse;

// Delete

// List

pub type ListRequest = QueryParameters;

#[derive(Debug, Serialize)]
pub struct ListResponse(Vec<ListResponseData>);

impl ListResponse {
    pub fn new(agents: &[models::RoomAgent]) -> ListResponse {
        let data: Vec<ListResponseData> = agents
            .iter()
            .map(|agent| ListResponseData {
                id: agent.agent_id,
                data: ReadResponseData {
                    label: agent.label.clone(),
                    created_at: agent.created_at,
                },
            })
            .collect();

        ListResponse(data)
    }
}

type ListResponseData = ReadResponse;

// List

// Join

#[derive(Debug, Deserialize)]
pub struct JoinRequest {
    pub room_id: Uuid,
    pub id: Uuid,
    pub data: JoinRequestData,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinRequestData {
    pub label: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct JoinResponse {
    id: Uuid,
    data: JoinResponseData,
}

#[derive(Clone, Debug, Serialize)]
struct JoinResponseData {
    label: String,
    created_at: NaiveDateTime,
}

impl JoinResponse {
    pub fn new(agent: &models::RoomAgent) -> JoinResponse {
        JoinResponse {
            id: agent.agent_id,
            data: JoinResponseData {
                label: agent.label.clone(),
                created_at: agent.created_at,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinEventPayload {
    agent_id: Uuid,
    room_id: Uuid,
}

impl JoinEventPayload {
    pub fn new(agent_id: Uuid, room_id: Uuid) -> JoinEventPayload {
        JoinEventPayload { agent_id, room_id }
    }
}

pub type JoinEvent = Event<JoinEventPayload>;

impl From<JoinEvent> for EventKind {
    fn from(event: JoinEvent) -> Self {
        EventKind::AgentJoin(event)
    }
}

// Join

// Leave

#[derive(Debug, Deserialize)]
pub struct LeaveRequest {
    pub room_id: Uuid,
    pub id: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LeaveResponse {
    id: Uuid,
    data: LeaveResponseData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct LeaveResponseData {
    label: String,
    created_at: NaiveDateTime,
}

impl LeaveResponse {
    pub fn new(agent: &models::RoomAgent) -> LeaveResponse {
        LeaveResponse {
            id: agent.agent_id,
            data: LeaveResponseData {
                label: agent.label.clone(),
                created_at: agent.created_at,
            },
        }
    }
}

pub type LeaveEvent = Event<LeaveResponse>;

impl From<LeaveEvent> for EventKind {
    fn from(event: LeaveEvent) -> Self {
        EventKind::AgentLeave(event)
    }
}

// Leave
