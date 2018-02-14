use models;

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
    pub fn build_response(self, rooms: &[models::Room]) -> RoomsListResponse {
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
