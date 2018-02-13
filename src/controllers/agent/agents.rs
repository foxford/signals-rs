use diesel;
use diesel::prelude::*;
use rumqtt::QoS;
use serde_json;
use std::env;

use controllers::{CrudlController, Response, TopicController};
use errors::*;
use messages::{Envelope, Message};
use models;
use schema::{agents, rooms};
use topic::{AgentTopic, Topic};

pub struct Controller;

impl TopicController for Controller {
    type Topic = AgentTopic;

    fn call(topic: &AgentTopic, envelope: Envelope) -> Result<Response> {
        let msg = envelope.message()?;

        if let Some(ref resource) = topic.resource {
            match resource.id {
                Some(agent_id) => {
                    // FIXME: use connection pool
                    let conn = establish_connection();

                    let agent: models::Agent = agents::table
                        .find(agent_id)
                        .first(&conn)
                        .map_err(|_| ErrorKind::NotFound)?;

                    match msg {
                        Message::AgentsReadRequest(_) => Controller::read(topic, envelope, agent),
                        Message::AgentsUpdateRequest(_) => {
                            Controller::update(topic, envelope, agent)
                        }
                        Message::AgentsDeleteRequest(_) => {
                            Controller::delete(topic, envelope, agent)
                        }
                        _ => unreachable!(),
                    }
                }
                None => match msg {
                    Message::AgentsCreateRequest(_) => Controller::create(topic, envelope),
                    Message::AgentsListRequest(_) => Controller::list(topic, envelope),
                    _ => Err(ErrorKind::BadRequest)?,
                },
            }
        } else {
            unreachable!()
        }
    }
}

impl CrudlController for Controller {
    type Topic = AgentTopic;
    type Resource = models::Agent;

    fn create(topic: &AgentTopic, envelope: Envelope) -> Result<Response> {
        // FIXME: use connection pool
        let conn = establish_connection();
        let msg = envelope.message()?;

        match msg {
            Message::AgentsCreateRequest(req) => {
                let room: models::Room = rooms::table
                    .find(topic.room_id.unwrap())
                    .first(&conn)
                    .map_err(|_| ErrorKind::NotFound)?;

                let changeset = models::NewAgent {
                    label: req.payload.label.clone(),
                    room_id: room.id,
                };

                let agent: models::Agent = {
                    diesel::insert_into(agents::table)
                        .values(&changeset)
                        .get_result(&conn)?
                };

                let resp = req.build_response(&agent);
                let resp = Message::AgentsCreateResponse(resp);
                let payload = serde_json::to_string(&resp).unwrap();
                let topic = topic.get_reverse();

                // TODO: grant permissions to allow agent to subscribe for apps topic

                Ok(Response {
                    topic: Topic::Agent(topic),
                    qos: QoS::Level1,
                    payload: payload.into_bytes(),
                })
            }
            _ => unreachable!(),
        }
    }

    fn read(topic: &AgentTopic, envelope: Envelope, agent: models::Agent) -> Result<Response> {
        let msg = envelope.message()?;

        match msg {
            Message::AgentsReadRequest(req) => {
                let resp = req.build_response(&agent);
                let resp = Message::AgentsReadResponse(resp);
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

    fn update(topic: &AgentTopic, envelope: Envelope, agent: models::Agent) -> Result<Response> {
        // FIXME: use connection pool
        let conn = establish_connection();
        let msg = envelope.message()?;

        match msg {
            Message::AgentsUpdateRequest(req) => {
                let changeset = models::NewAgent {
                    label: req.payload.label.clone(),
                    // TODO: don't reassign room_id
                    room_id: agent.room_id,
                };
                let agent: models::Agent =
                    diesel::update(&agent).set(&changeset).get_result(&conn)?;

                let resp = req.build_response(&agent);
                let resp = Message::AgentsUpdateResponse(resp);
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

    fn delete(topic: &AgentTopic, envelope: Envelope, agent: models::Agent) -> Result<Response> {
        // FIXME: use connection pool
        let conn = establish_connection();
        let msg = envelope.message()?;

        match msg {
            Message::AgentsDeleteRequest(req) => {
                diesel::delete(&agent).execute(&conn)?;

                let resp = req.build_response(&agent);
                let resp = Message::AgentsDeleteResponse(resp);
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
            Message::AgentsListRequest(req) => {
                let room: models::Room = rooms::table
                    .find(topic.room_id.unwrap())
                    .first(&conn)
                    .map_err(|_| ErrorKind::NotFound)?;

                let agents = models::Agent::belonging_to(&room).load::<models::Agent>(&conn)?;

                let resp = req.build_response(&agents);
                let resp = Message::AgentsListResponse(resp);
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
