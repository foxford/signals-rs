use models;
use uuid::Uuid;

// Create

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub data: CreateRequestData,
}

pub type CreateRequestData = models::NewRoom;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    id: String,
    data: CreateResponseData,
}

type CreateResponseData = CreateRequestData;

impl CreateResponse {
    pub fn new(room: &models::Room) -> CreateResponse {
        CreateResponse {
            id: room.id.to_string(),
            data: CreateResponseData {
                label: Some(room.label.clone()),
            },
        }
    }
}
// Create

// Read

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadRequest {
    pub room_id: Uuid,
}

pub type ReadResponse = CreateResponse;
type ReadResponseData = CreateResponseData;

// Read

// Update

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRequest {
    pub room_id: Uuid,
    pub data: CreateRequestData,
}

pub type UpdateResponse = CreateResponse;

// Update

// Delete

pub type DeleteRequest = ReadRequest;
pub type DeleteResponse = ReadResponse;

// Delete

// List

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse(Vec<ListResponseData>);

impl ListResponse {
    pub fn new(rooms: &[models::Room]) -> ListResponse {
        let data: Vec<ListResponseData> = rooms
            .iter()
            .map(|room| ListResponseData {
                id: room.id.to_string(),
                data: ReadResponseData {
                    label: Some(room.label.clone()),
                },
            })
            .collect();

        ListResponse(data)
    }
}

type ListResponseData = ReadResponse;

// List
