use chrono::NaiveDateTime;
use uuid::Uuid;

use models;

// Create

#[derive(Debug, Serialize)]
pub struct CreateResponse {
    id: Uuid,
    data: CreateResponseData,
}

impl CreateResponse {
    pub fn new(room: &models::Room) -> CreateResponse {
        CreateResponse {
            id: room.id,
            data: CreateResponseData {
                created_at: room.created_at,
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct CreateResponseData {
    created_at: NaiveDateTime,
}

// Create

// Read

#[derive(Debug, Deserialize)]
pub struct ReadRequest {
    pub room_id: Uuid,
}

pub type ReadResponse = CreateResponse;
type ReadResponseData = CreateResponseData;

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
            .map(|room| ListResponseData {
                id: room.id,
                data: ReadResponseData {
                    created_at: room.created_at,
                },
            })
            .collect();

        ListResponse(data)
    }
}

type ListResponseData = ReadResponse;

// List
