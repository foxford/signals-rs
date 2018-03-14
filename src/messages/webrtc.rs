use jsonrpc_core::{Notification, Params, Version};
use serde_json;
use uuid::Uuid;

use messages;

#[derive(Debug, Deserialize)]
pub struct Request<T> {
    room_id: Uuid,
    data: T,
}

#[derive(Debug, Serialize)]
pub struct NotificationParams<T> {
    room_id: Uuid,
    data: T,
}

// Offer

pub type OfferRequest = Request<OfferRequestData>;

#[derive(Debug, Deserialize)]
pub struct OfferRequestData {
    jsep: serde_json::Value,
    from: Uuid,
    to: Uuid,
    tracks: Vec<Track>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Track {
    id: Uuid,
}

type OfferNotification = NotificationParams<OfferNotificationData>;

#[derive(Debug, Serialize)]
struct OfferNotificationData {
    jsep: serde_json::Value,
    from: Uuid,
    tracks: Vec<Track>,
}

impl From<OfferRequest> for OfferNotificationData {
    fn from(req: OfferRequest) -> Self {
        OfferNotificationData {
            jsep: req.data.jsep,
            from: req.data.from,
            tracks: req.data.tracks,
        }
    }
}

// Offer

// Answer

pub type AnswerRequest = Request<AnswerRequestData>;

#[derive(Debug, Deserialize)]
pub struct AnswerRequestData {
    jsep: serde_json::Value,
    from: Uuid,
    to: Uuid,
}

type AnswerNotification = NotificationParams<AnswerNotificationData>;

#[derive(Debug, Serialize)]
struct AnswerNotificationData {
    jsep: serde_json::Value,
    from: Uuid,
}

impl From<AnswerRequest> for AnswerNotificationData {
    fn from(req: AnswerRequest) -> Self {
        AnswerNotificationData {
            jsep: req.data.jsep,
            from: req.data.from,
        }
    }
}

// Answer

// Candidate

pub type CandidateRequest = Request<CandidateRequestData>;

#[derive(Debug, Deserialize)]
pub struct CandidateRequestData {
    candidate: serde_json::Value,
    from: Uuid,
    to: Uuid,
}

type CandidateNotification = NotificationParams<CandidateNotificationData>;

#[derive(Debug, Serialize)]
struct CandidateNotificationData {
    candidate: serde_json::Value,
    from: Uuid,
}

impl From<CandidateRequest> for CandidateNotificationData {
    fn from(req: CandidateRequest) -> Self {
        CandidateNotificationData {
            candidate: req.data.candidate,
            from: req.data.from,
        }
    }
}

// Candidate

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum WebrtcMethod {
    Offer(OfferNotification),
    Answer(AnswerNotification),
    Candidate(CandidateNotification),
}

impl From<OfferRequest> for WebrtcMethod {
    fn from(req: OfferRequest) -> Self {
        let offer = OfferNotification {
            room_id: req.room_id,
            data: req.into(),
        };
        WebrtcMethod::Offer(offer)
    }
}

impl From<AnswerRequest> for WebrtcMethod {
    fn from(req: AnswerRequest) -> Self {
        let answer = AnswerNotification {
            room_id: req.room_id,
            data: req.into(),
        };
        WebrtcMethod::Answer(answer)
    }
}

impl From<CandidateRequest> for WebrtcMethod {
    fn from(req: CandidateRequest) -> Self {
        let candidate = CandidateNotification {
            room_id: req.room_id,
            data: req.into(),
        };
        WebrtcMethod::Candidate(candidate)
    }
}

impl From<WebrtcMethod> for Option<Params> {
    fn from(method: WebrtcMethod) -> Self {
        serde_json::to_value(method)
            .ok()
            .map(|value| Params::Array(vec![value]))
    }
}

impl From<WebrtcMethod> for Notification {
    fn from(method: WebrtcMethod) -> Self {
        let method_name = match method {
            WebrtcMethod::Offer(_) => "webrtc.offer",
            WebrtcMethod::Answer(_) => "webrtc.answer",
            WebrtcMethod::Candidate(_) => "webrtc.candidate",
        };

        Notification {
            jsonrpc: Some(Version::V2),
            method: method_name.to_owned(),
            params: method.into(),
        }
    }
}

impl From<OfferRequest> for Notification {
    fn from(req: OfferRequest) -> Self {
        let method = WebrtcMethod::from(req);
        Notification::from(method)
    }
}

impl From<AnswerRequest> for Notification {
    fn from(req: AnswerRequest) -> Self {
        let method = WebrtcMethod::from(req);
        Notification::from(method)
    }
}

impl From<CandidateRequest> for Notification {
    fn from(req: CandidateRequest) -> Self {
        let method = WebrtcMethod::from(req);
        Notification::from(method)
    }
}

impl From<OfferRequest> for messages::Method {
    fn from(req: OfferRequest) -> Self {
        messages::Method {
            agent_id: req.data.to,
            body: req.into(),
        }
    }
}

impl From<AnswerRequest> for messages::Method {
    fn from(req: AnswerRequest) -> Self {
        messages::Method {
            agent_id: req.data.to,
            body: req.into(),
        }
    }
}

impl From<CandidateRequest> for messages::Method {
    fn from(req: CandidateRequest) -> Self {
        messages::Method {
            agent_id: req.data.to,
            body: req.into(),
        }
    }
}
