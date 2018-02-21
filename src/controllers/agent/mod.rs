use controllers::{Response, TopicController};
use controllers::agent::agents::Controller as AgentsController;

use errors::*;
use messages::Envelope;
use topic::{AgentTopic, ResourceKind};

mod agents;

pub struct AgentController;

impl TopicController for AgentController {
    type Topic = AgentTopic;

    fn call(topic: &AgentTopic, envelope: Envelope) -> Result<Vec<Response>> {
        match topic.resource {
            Some(ref resource) => match resource.kind {
                ResourceKind::Agents => AgentsController::call(topic, envelope),
                _ => unimplemented!(),
            },
            None => unimplemented!(),
        }
    }
}
