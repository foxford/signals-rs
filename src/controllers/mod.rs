use rumqtt::QoS;

// use controllers::agent::AgentController;
use controllers::ping::PingController;
use errors::*;
use messages::Envelope;
use topic::{Reversible, Topic};

// mod agent;
mod ping;

pub struct Response {
    pub topic: Topic,
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

    pub fn call(&self, envelope: Envelope) -> Result<Vec<Response>> {
        match *self.topic {
            Topic::Ping(ref t) => PingController::call(t, envelope),
            Topic::Agent(_) => unimplemented!(),
            Topic::App(_) => unreachable!(),
        }
    }
}

trait TopicController {
    type Topic: Reversible;

    fn call(topic: &Self::Topic, envelope: Envelope) -> Result<Vec<Response>>;
}

trait CrudlController {
    type Topic: Reversible;
    type Resource;

    fn create(topic: &Self::Topic, envelope: Envelope) -> Result<Vec<Response>>;
    fn read(
        topic: &Self::Topic,
        envelope: Envelope,
        resource: Self::Resource,
    ) -> Result<Vec<Response>>;
    fn update(
        topic: &Self::Topic,
        envelope: Envelope,
        resource: Self::Resource,
    ) -> Result<Vec<Response>>;
    fn delete(
        topic: &Self::Topic,
        envelope: Envelope,
        resource: Self::Resource,
    ) -> Result<Vec<Response>>;
    fn list(topic: &Self::Topic, envelope: Envelope) -> Result<Vec<Response>>;
}
