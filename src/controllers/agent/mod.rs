use controllers::{Response, TopicController};
use controllers::agent::agents::Controller as AgentsController;
use controllers::agent::rooms::Controller as RoomController;

use errors::*;
use messages::Envelope;
use topic::{AgentTopic, ResourceKind};

mod agents;
mod rooms;

pub struct AgentController;

impl TopicController for AgentController {
    type Topic = AgentTopic;

    fn call(topic: &AgentTopic, envelope: Envelope) -> Result<Response> {
        match topic.resource {
            Some(ref resource) => match resource.kind {
                ResourceKind::Agents => AgentsController::call(topic, envelope),
                _ => unimplemented!(),
            },
            None => RoomController::call(topic, envelope),
        }
    }
}
