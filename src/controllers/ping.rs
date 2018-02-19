use rumqtt::QoS;
use serde_json;

use controllers::{Response, TopicController};
use errors::*;
use messages::{Envelope, Message};
use topic::{PingTopicKind, Reversible, Topic};

pub struct PingController;

impl TopicController for PingController {
    type Topic = PingTopicKind;

    fn call(topic: &PingTopicKind, envelope: Envelope) -> Result<Vec<Response>> {
        let msg = Message::from_envelope(&envelope)?;
        match msg {
            Message::Ping => {
                let payload = serde_json::to_string(&Message::Pong).unwrap();
                let topic = topic.get_reverse();

                Ok(vec![
                    Response {
                        topic: Topic::Ping(topic),
                        qos: QoS::Level0,
                        payload: payload.into_bytes(),
                    },
                ])
            }
            _ => unimplemented!(),
        }
    }
}
