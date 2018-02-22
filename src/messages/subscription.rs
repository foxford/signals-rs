use uuid::Uuid;

use topic::{AppTopic, ResourceKind};

// Create

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub room_id: Uuid,
    pub agent_id: Uuid,
    pub data: CreateRequestData,
}

#[derive(Debug, Deserialize)]
pub struct CreateRequestData {
    pub resource: ResourceKind,
}

#[derive(Debug, Serialize)]
pub struct CreateResponse {
    data: CreateResponseData,
}

impl CreateResponse {
    pub fn new(topic: AppTopic) -> CreateResponse {
        CreateResponse {
            data: CreateResponseData { topic },
        }
    }
}

#[derive(Debug, Serialize)]
struct CreateResponseData {
    topic: AppTopic,
}

// Create
