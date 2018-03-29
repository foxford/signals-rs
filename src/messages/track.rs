use serde_json::Value;
use uuid::Uuid;

use messages::query_parameters::QueryParameters;
use messages::{Event, EventKind};
use models;

// Create

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub room_id: Uuid,
    pub data: CreateRequestData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateRequestData {
    pub owner_id: Uuid,
    pub metadata: Value,
}

#[derive(Clone, Debug, Serialize)]
pub struct CreateResponse {
    id: Uuid,
    data: CreateResponseData,
}

impl CreateResponse {
    pub fn new(track: &models::Track) -> CreateResponse {
        CreateResponse {
            id: track.id,
            data: CreateResponseData::new(track),
        }
    }
}

type CreateResponseData = CreateRequestData;

impl CreateResponseData {
    fn new(track: &models::Track) -> CreateResponseData {
        CreateResponseData {
            owner_id: track.owner_id,
            metadata: track.metadata.clone(),
        }
    }
}

pub type CreateEvent = Event<CreateResponse>;

impl From<CreateEvent> for EventKind {
    fn from(event: CreateEvent) -> Self {
        EventKind::TrackCreate(event)
    }
}

// Create

// Delete

#[derive(Debug, Deserialize)]
pub struct DeleteRequest {
    pub room_id: Uuid,
    pub id: Uuid,
}

#[derive(Clone, Debug, Serialize)]
pub struct DeleteResponse {
    id: Uuid,
    data: DeleteResponseData,
}

type DeleteResponseData = CreateResponseData;

impl DeleteResponse {
    pub fn new(track: &models::Track) -> DeleteResponse {
        DeleteResponse {
            id: track.id,
            data: DeleteResponseData::new(track),
        }
    }
}

pub type DeleteEvent = Event<DeleteResponse>;

impl From<DeleteEvent> for EventKind {
    fn from(event: DeleteEvent) -> Self {
        EventKind::TrackDelete(event)
    }
}

// Delete

// List

pub type ListRequest = QueryParameters;

#[derive(Debug, Serialize)]
pub struct ListResponse(Vec<ListResponseData>);

impl ListResponse {
    pub fn new(tracks: &[models::Track]) -> ListResponse {
        let data: Vec<ListResponseData> = tracks
            .iter()
            .map(|track| ListResponseData::new(track))
            .collect();

        ListResponse(data)
    }
}

type ListResponseData = CreateResponse;

// List
