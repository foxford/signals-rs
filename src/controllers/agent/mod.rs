use controllers::{Response, TopicController};
use controllers::agent::rooms::Controller as RoomController;

use errors::*;
use messages::Envelope;
use topic::AgentTopic;

mod rooms;

pub struct AgentController;

impl TopicController for AgentController {
    type Topic = AgentTopic;

    fn call(topic: &AgentTopic, envelope: Envelope) -> Result<Response> {
        // TODO: agent and track controllers
        RoomController::call(topic, envelope)
    }
}
