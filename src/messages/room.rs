use models;
use uuid::Uuid;

// Create

#[derive(Debug, Serialize)]
pub struct CreateResponse {
    id: Uuid,
}

impl CreateResponse {
    pub fn new(room: &models::Room) -> CreateResponse {
        CreateResponse { id: room.id }
    }
}

// Create

// Read

#[derive(Debug, Deserialize)]
pub struct ReadRequest {
    pub room_id: Uuid,
}

pub type ReadResponse = CreateResponse;

// Read

// Delete

pub type DeleteRequest = ReadRequest;
pub type DeleteResponse = ReadResponse;

// Delete

// List

#[derive(Debug, Serialize)]
pub struct ListResponse(Vec<ListResponseData>);

impl ListResponse {
    pub fn new(rooms: &[models::Room]) -> ListResponse {
        let data: Vec<ListResponseData> = rooms
            .iter()
            .map(|room| ListResponseData { id: room.id })
            .collect();

        ListResponse(data)
    }
}

type ListResponseData = ReadResponse;

// List
