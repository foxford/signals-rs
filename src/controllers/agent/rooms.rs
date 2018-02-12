use diesel;
use diesel::prelude::*;
use rumqtt::QoS;
use serde_json;
use std::env;

use controllers::{CrudlController, Response, TopicController};
use errors::*;
use messages::{Envelope, Message};
use models;
use schema::rooms;
use topic::{AgentTopic, Reversible, Topic};

pub struct Controller;

impl TopicController for Controller {
    type Topic = AgentTopic;

    fn call(topic: &AgentTopic, envelope: Envelope) -> Result<Response> {
        let msg = envelope.message()?;

        match topic.room_id {
            Some(room_id) => {
                // FIXME: use connection pool
                let conn = establish_connection();

                let room: models::Room = rooms::table
                    .find(room_id)
                    .first(&conn)
                    .map_err(|_| ErrorKind::NotFound)?;

                match msg {
                    Message::RoomsReadRequest(_) => Controller::read(topic, envelope, room),
                    Message::RoomsUpdateRequest(_) => Controller::update(topic, envelope, room),
                    Message::RoomsDeleteRequest(_) => Controller::delete(topic, envelope, room),
                    _ => unreachable!(),
                }
            }
            None => match msg {
                Message::RoomsCreateRequest(_) => Controller::create(topic, envelope),
                Message::RoomsListRequest(_) => Controller::list(topic, envelope),
                _ => Err(ErrorKind::BadRequest)?,
            },
        }
    }
}

impl CrudlController for Controller {
    type Topic = AgentTopic;
    type Resource = models::Room;

    fn create(topic: &AgentTopic, envelope: Envelope) -> Result<Response> {
        // FIXME: use connection pool
        let conn = establish_connection();
        let msg = envelope.message()?;

        match msg {
            Message::RoomsCreateRequest(req) => {
                let room: models::Room = {
                    diesel::insert_into(rooms::table)
                        .values(&req.payload)
                        .get_result(&conn)?
                };

                let resp = req.build_response(&room);
                let resp = Message::RoomsCreateResponse(resp);
                let payload = serde_json::to_string(&resp).unwrap();

                let topic = topic.get_reverse();
                Ok(Response {
                    topic: Topic::Agent(topic),
                    qos: QoS::Level1,
                    payload: payload.into_bytes(),
                })
            }
            _ => unreachable!(),
        }
    }

    fn read(topic: &AgentTopic, envelope: Envelope, room: models::Room) -> Result<Response> {
        let msg = envelope.message()?;

        match msg {
            Message::RoomsReadRequest(req) => {
                let resp = req.build_response(&room);
                let resp = Message::RoomsReadResponse(resp);
                let payload = serde_json::to_string(&resp).unwrap();

                let topic = topic.get_reverse();
                Ok(Response {
                    topic: Topic::Agent(topic),
                    qos: QoS::Level1,
                    payload: payload.into_bytes(),
                })
            }
            _ => unreachable!(),
        }
    }

    fn update(topic: &AgentTopic, envelope: Envelope, room: models::Room) -> Result<Response> {
        // FIXME: use connection pool
        let conn = establish_connection();
        let msg = envelope.message()?;

        match msg {
            Message::RoomsUpdateRequest(req) => {
                let room: models::Room = diesel::update(&room).set(&req.payload).get_result(&conn)?;

                let resp = req.build_response(&room);
                let resp = Message::RoomsUpdateResponse(resp);
                let payload = serde_json::to_string(&resp).unwrap();

                let topic = topic.get_reverse();
                Ok(Response {
                    topic: Topic::Agent(topic),
                    qos: QoS::Level1,
                    payload: payload.into_bytes(),
                })
            }
            _ => unreachable!(),
        }
    }

    fn delete(topic: &AgentTopic, envelope: Envelope, room: models::Room) -> Result<Response> {
        // FIXME: use connection pool
        let conn = establish_connection();
        let msg = envelope.message()?;

        match msg {
            Message::RoomsDeleteRequest(req) => {
                diesel::delete(&room).execute(&conn)?;

                let resp = req.build_response(&room);
                let resp = Message::RoomsDeleteResponse(resp);
                let payload = serde_json::to_string(&resp).unwrap();

                let topic = topic.get_reverse();
                Ok(Response {
                    topic: Topic::Agent(topic),
                    qos: QoS::Level1,
                    payload: payload.into_bytes(),
                })
            }
            _ => unreachable!(),
        }
    }

    fn list(topic: &AgentTopic, envelope: Envelope) -> Result<Response> {
        // FIXME: use connection pool
        let conn = establish_connection();
        let msg = envelope.message()?;

        match msg {
            Message::RoomsListRequest(req) => {
                let rooms = rooms::table.load::<models::Room>(&conn)?;

                let resp = req.build_response(&rooms);
                let resp = Message::RoomsListResponse(resp);
                let payload = serde_json::to_string(&resp).unwrap();

                let topic = topic.get_reverse();
                Ok(Response {
                    topic: Topic::Agent(topic),
                    qos: QoS::Level1,
                    payload: payload.into_bytes(),
                })
            }
            _ => unreachable!(),
        }
    }
}

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap()
}
