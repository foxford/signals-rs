use rumqtt::QoS;

use controllers::agent::AgentController;
use controllers::ping::PingController;
use errors::*;
use messages::Envelope;
use topic::Topic;

mod agent;
mod ping;

pub struct Response {
    // TODO: try with Topic type
    pub topic: String,
    pub qos: QoS,
    pub payload: Vec<u8>,
}

pub struct MainController<'a> {
    topic: &'a Topic,
}

impl<'a> MainController<'a> {
    pub fn new(topic: &Topic) -> MainController {
        MainController { topic }
    }

    pub fn call(&self, envelope: Envelope) -> Result<Response> {
        match *self.topic {
            Topic::Ping => PingController::call(&(), envelope),
            Topic::Agent(ref topic) => AgentController::call(topic, envelope),
            _ => unimplemented!(),
        }
    }
}

trait TopicController {
    type Topic;

    fn call(topic: &Self::Topic, envelope: Envelope) -> Result<Response>;
}

trait CrudlController {
    type Topic;
    type Resource;

    fn create(topic: &Self::Topic, envelope: Envelope) -> Result<Response>;
    fn read(topic: &Self::Topic, envelope: Envelope, resource: Self::Resource) -> Result<Response>;
    fn update(
        topic: &Self::Topic,
        envelope: Envelope,
        resource: Self::Resource,
    ) -> Result<Response>;
    fn delete(
        topic: &Self::Topic,
        envelope: Envelope,
        resource: Self::Resource,
    ) -> Result<Response>;
    fn list(topic: &Self::Topic, envelope: Envelope) -> Result<Response>;
}
