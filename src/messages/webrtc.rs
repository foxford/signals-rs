use serde_json;
use uuid::Uuid;

// Offer

#[derive(Debug, Deserialize)]
pub struct OfferRequest {
    room_id: Uuid,
    data: OfferRequestData,
}

#[derive(Debug, Deserialize)]
struct OfferRequestData {
    jsep: serde_json::Value,
    to: Uuid,
    tracks: Vec<Track>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Track {
    id: Uuid
}

#[derive(Debug, Serialize)]
pub struct OfferNotification {
    room_id: Uuid,
    data: OfferNotificationData,
}

#[derive(Debug, Serialize)]
struct OfferNotificationData {
    jsep: serde_json::Value,
    from: Uuid,
    tracks: Vec<Track>,
}

// Offer
