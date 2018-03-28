use models;
use uuid::Uuid;

use messages::query_parameters::QueryParameters;
use messages::{Event, EventKind};

// Create

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub room_id: Uuid,
    pub data: CreateRequestData,
}

type CreateRequestData = models::NewLocalTrack;
#[derive(Clone, Debug, Serialize)]
pub struct CreateResponse {
    id: Uuid,
    data: CreateResponseData,
}

#[derive(Clone, Debug, Serialize)]
struct CreateResponseData {
    pub stream_id: String,
    pub track_id: String,
    pub device: String,
    pub kind: String,
    pub label: String,
    pub owner_id: Uuid,
    pub holders: Vec<Holder>,
}

#[derive(Clone, Debug, Serialize)]
struct Holder {
    id: Uuid,
}

impl CreateResponse {
    pub fn new(
        track: &models::LocalTrack,
        remote_tracks: &[models::RemoteTrack],
    ) -> CreateResponse {
        CreateResponse {
            id: track.id,
            data: CreateResponseData::new(track, remote_tracks),
        }
    }
}

impl CreateResponseData {
    fn new(
        track: &models::LocalTrack,
        remote_tracks: &[models::RemoteTrack],
    ) -> CreateResponseData {
        CreateResponseData {
            stream_id: track.stream_id.clone(),
            track_id: track.track_id.clone(),
            device: track.device.clone(),
            kind: track.kind.clone(),
            label: track.label.clone(),
            owner_id: track.owner_id,
            holders: remote_tracks
                .iter()
                .map(|t| Holder { id: t.agent_id })
                .collect(),
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

// Update

#[derive(Clone, Debug, Serialize)]
pub struct UpdateResponse {
    id: Uuid,
    data: UpdateResponseData,
}

impl UpdateResponse {
    pub fn new(
        track: &models::LocalTrack,
        remote_tracks: &[models::RemoteTrack],
    ) -> UpdateResponse {
        UpdateResponse {
            id: track.id,
            data: UpdateResponseData::new(track, remote_tracks),
        }
    }
}

type UpdateResponseData = CreateResponseData;

pub type UpdateEvent = Event<UpdateResponse>;

impl From<UpdateEvent> for EventKind {
    fn from(event: UpdateEvent) -> Self {
        EventKind::TrackUpdate(event)
    }
}

// Update

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
    pub fn new(
        track: &models::LocalTrack,
        remote_tracks: &[models::RemoteTrack],
    ) -> DeleteResponse {
        DeleteResponse {
            id: track.id,
            data: DeleteResponseData::new(track, remote_tracks),
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

// Register

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub room_id: Uuid,
    pub data: RegisterRequestData,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequestData {
    pub stream_id: String,
    pub track_id: String,
    pub agent_id: Uuid,
}

pub type RegisterResponse = UpdateResponse;

// Register

// Unregister

pub type UnregisterRequest = RegisterRequest;
pub type UnregisterResponse = RegisterResponse;

// Unregister

// List

pub type ListRequest = QueryParameters;

#[derive(Debug, Serialize)]
pub struct ListResponse(Vec<ListResponseData>);

impl ListResponse {
    pub fn new(tracks: &[(models::LocalTrack, Vec<models::RemoteTrack>)]) -> ListResponse {
        let data: Vec<ListResponseData> = tracks
            .iter()
            .map(|track| ListResponseData::new(&track.0, &track.1))
            .collect();

        ListResponse(data)
    }
}

type ListResponseData = CreateResponse;

// List
