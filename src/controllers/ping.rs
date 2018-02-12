use rumqtt::QoS;
use serde_json;

use controllers::{Response, TopicController};
use errors::*;
use messages::{Envelope, Message};

pub struct PingController;

impl TopicController for PingController {
    type Topic = ();

    fn call(_topic: &(), envelope: Envelope) -> Result<Response> {
        let msg = envelope.message()?;
        match msg {
            Message::Ping => {
                let payload = serde_json::to_string(&Message::Pong).unwrap();

                Ok(Response {
                    topic: "pong".to_string(),
                    qos: QoS::Level0,
                    payload: payload.into_bytes(),
                })
            }
            _ => unimplemented!(),
        }
    }
}
