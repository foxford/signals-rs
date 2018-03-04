use models;
use uuid::Uuid;

use messages::{Event, EventKind};
use messages::query_parameters::QueryParameters;

// Create

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub room_id: Uuid,
    pub id: Uuid,
    pub data: CreateRequestData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateRequestData {
    pub label: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    id: Uuid,
    data: CreateResponseData,
}

type CreateResponseData = CreateRequestData;

impl CreateResponse {
    pub fn new(agent: &models::Agent) -> CreateResponse {
        CreateResponse {
            id: agent.id,
            data: CreateResponseData {
                label: agent.label.clone(),
            },
        }
    }
}

pub type CreateEvent = Event<CreateResponse>;

impl From<CreateEvent> for EventKind {
    fn from(event: CreateEvent) -> Self {
        EventKind::AgentCreate(event)
    }
}

// Create

// Read

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadRequest {
    pub room_id: Uuid,
    pub id: Uuid,
}

pub type ReadResponse = CreateResponse;
type ReadResponseData = CreateResponseData;

// Read

// Update

pub type UpdateRequest = CreateRequest;
pub type UpdateResponse = CreateResponse;

// Update

// Delete

pub type DeleteRequest = ReadRequest;

#[derive(Clone, Debug, Serialize)]
pub struct DeleteResponse {
    id: Uuid,
    data: DeleteResponseData,
}

type DeleteResponseData = ReadResponseData;

impl DeleteResponse {
    pub fn new(agent: &models::Agent) -> DeleteResponse {
        DeleteResponse {
            id: agent.id,
            data: DeleteResponseData {
                label: agent.label.clone(),
            },
        }
    }
}

pub type DeleteEvent = Event<DeleteResponse>;

impl From<DeleteEvent> for EventKind {
    fn from(event: DeleteEvent) -> Self {
        EventKind::AgentDelete(event)
    }
}

// Delete

// List

pub type ListRequest = QueryParameters;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse(Vec<ListResponseData>);

impl ListResponse {
    pub fn new(agents: &[models::Agent]) -> ListResponse {
        let data: Vec<ListResponseData> = agents
            .iter()
            .map(|agent| ListResponseData {
                id: agent.id,
                data: ReadResponseData {
                    label: agent.label.clone(),
                },
            })
            .collect();

        ListResponse(data)
    }
}

type ListResponseData = ReadResponse;

// List
