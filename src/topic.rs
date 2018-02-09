use nom::{alphanumeric, is_hex_digit};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

use errors::*;

named!(ping_topic<&str, Topic>,
    alt!(
        map!(tag_s!("ping"), |_| Topic::Ping(PingTopicKind::Ping)) |
        map!(tag_s!("pong"), |_| Topic::Ping(PingTopicKind::Pong))
    )
);

named!(agent_topic<&str, Topic>,
    do_parse!(
        tag_s!("agents/") >>
        agent_id: uuid >>
        tag_s!("/") >>
        kind: alt!(map!(tag_s!("in"), |_| AgentTopicKind::In) | map!(tag_s!("out"), |_| AgentTopicKind::Out)) >>
        tag_s!("/signals.netology-group.services/api/") >>
        version: map_res!(alphanumeric, FromStr::from_str) >>
        tag_s!("/rooms") >>
        room_id: opt!(complete!(preceded!(
            tag_s!("/"),
            uuid
        ))) >>
        resource: opt!(complete!(preceded!(
            tag_s!("/"),
            topic_resource
        ))) >>

        (Topic::Agent(AgentTopic { kind, agent_id, version, room_id, resource }))
    )
);

named!(topic_resource<&str, Resource>,
    do_parse!(
        kind: alt!(map!(tag_s!("agents"), |_| ResourceKind::Agents) | map!(tag_s!("tracks"), |_| ResourceKind::Tracks)) >>
        id: opt!(complete!(preceded!(
            tag_s!("/"),
            uuid
        ))) >>

        (Resource { kind, id } )
    )
);

named!(topic<&str, Topic>,
    alt!(ping_topic | agent_topic)
);

named!(uuid<&str, Uuid>,
    map_res!(recognize!(tuple!(
        verify!(take_while_s!(|chr| is_hex_digit(chr as u8)), |s: &str| s.len() == 8 ),
        tag_s!("-"),
        verify!(take_while_s!(|chr| is_hex_digit(chr as u8)), |s: &str| s.len() == 4 ),
        tag_s!("-"),
        verify!(take_while_s!(|chr| is_hex_digit(chr as u8)), |s: &str| s.len() == 4 ),
        tag_s!("-"),
        verify!(take_while_s!(|chr| is_hex_digit(chr as u8)), |s: &str| s.len() == 4 ),
        tag_s!("-"),
        verify!(take_while_s!(|chr| is_hex_digit(chr as u8)), |s: &str| s.len() == 12 )
    )), FromStr::from_str)
);

#[derive(Debug, PartialEq)]
pub enum Topic {
    Ping(PingTopicKind),
    Agent(AgentTopic),
}

impl Topic {
    pub fn parse(topic_str: &str) -> Result<Topic> {
        Ok(topic(topic_str).to_result()?)
    }
}

impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: &fmt::Display = match *self {
            Topic::Ping(ref t) => t,
            Topic::Agent(ref t) => t,
        };

        write!(f, "{}", value)
    }
}

#[derive(Debug, PartialEq)]
pub enum PingTopicKind {
    Ping,
    Pong,
}

impl fmt::Display for PingTopicKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = format!("{:?}", self).to_lowercase();
        f.write_str(&value)
    }
}

#[derive(Debug, PartialEq)]
pub struct AgentTopic {
    kind: AgentTopicKind,
    agent_id: Uuid,
    version: String,
    pub room_id: Option<Uuid>,
    resource: Option<Resource>,
}

impl AgentTopic {
    pub fn get_reverse(&self) -> AgentTopic {
        let kind = match self.kind {
            AgentTopicKind::In => AgentTopicKind::Out,
            AgentTopicKind::Out => AgentTopicKind::In,
        };

        AgentTopic {
            kind,
            agent_id: self.agent_id.clone(),
            version: self.version.clone(),
            room_id: self.room_id.clone(),
            resource: self.resource.clone(),
        }
    }
}

impl fmt::Display for AgentTopic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind = self.kind.to_string().to_lowercase();
        let mut topic = format!(
            "agents/{}/{}/signals.netology-group.services/api/{}/rooms",
            self.agent_id, kind, self.version
        );

        if let Some(room_id) = self.room_id {
            topic.push_str("/");
            topic.push_str(&room_id.hyphenated().to_string());
        }

        if let Some(ref resource) = self.resource {
            topic.push_str("/");
            topic.push_str(&resource.to_string());
        }

        f.write_str(&topic)
    }
}

#[derive(Debug, PartialEq)]
enum AgentTopicKind {
    In,
    Out,
}

impl fmt::Display for AgentTopicKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Resource {
    kind: ResourceKind,
    id: Option<Uuid>,
}

#[derive(Clone, Debug, PartialEq)]
enum ResourceKind {
    Agents,
    Tracks,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind = self.kind.to_string().to_lowercase();
        let mut resource = format!("{}", kind);

        if let Some(id) = self.id {
            resource.push_str("/");
            resource.push_str(&id.hyphenated().to_string());
        }

        f.write_str(&resource)
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait Reversible {
    type Topic;

    fn get_reverse(&self) -> Self::Topic;
}

impl Reversible for PingTopicKind {
    type Topic = PingTopicKind;

    fn get_reverse(&self) -> PingTopicKind {
        match *self {
            PingTopicKind::Ping => PingTopicKind::Pong,
            PingTopicKind::Pong => PingTopicKind::Ping,
        }
    }
}

impl Reversible for AgentTopic {
    type Topic = AgentTopic;

    fn get_reverse(&self) -> AgentTopic {
        let kind = match self.kind {
            AgentTopicKind::In => AgentTopicKind::Out,
            AgentTopicKind::Out => AgentTopicKind::In,
        };

        AgentTopic {
            kind,
            agent_id: self.agent_id.clone(),
            version: self.version.clone(),
            room_id: self.room_id.clone(),
            resource: self.resource.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::Done;

    #[test]
    fn parse_agent_topic() {
        let topic = agent_topic("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1/rooms");
        let topic_exp = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: "v1".to_string(),
            room_id: None,
            resource: None,
        });
        assert_eq!(topic, Done("", topic_exp));

        let topic = agent_topic("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1/rooms/");
        let topic_exp = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: "v1".to_string(),
            room_id: None,
            resource: None,
        });
        assert_eq!(topic, Done("/", topic_exp));

        let topic = agent_topic("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1/rooms/058df470-73ea-43a4-b36c-e4615cad468e");
        let topic_exp = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: "v1".to_string(),
            room_id: Some(Uuid::parse_str("058df470-73ea-43a4-b36c-e4615cad468e").unwrap()),
            resource: None,
        });
        assert_eq!(topic, Done("", topic_exp));

        let topic =
            agent_topic("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals/api/v1/rooms/456");
        assert!(topic.is_err());
    }

    #[test]
    fn parse_topic_resource() {
        let resource = topic_resource("agents");
        let resource_exp = Resource {
            kind: ResourceKind::Agents,
            id: None,
        };
        assert_eq!(resource, Done("", resource_exp));

        let resource = topic_resource("agents/");
        let resource_exp = Resource {
            kind: ResourceKind::Agents,
            id: None,
        };
        assert_eq!(resource, Done("/", resource_exp));

        let resource = topic_resource("agents/c6d2eec6-94ac-4575-9658-10c93b939d9a");
        let resource_exp = Resource {
            kind: ResourceKind::Agents,
            id: Some(Uuid::parse_str("c6d2eec6-94ac-4575-9658-10c93b939d9a").unwrap()),
        };
        assert_eq!(resource, Done("", resource_exp));

        assert!(topic_resource("foo").is_err());
        assert!(topic_resource("foo/c6d2eec6-94ac-4575-9658-10c93b939d9a").is_err());
    }

    #[test]
    fn parse_agent_topic_with_resource() {
        let topic = agent_topic("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1/rooms/058df470-73ea-43a4-b36c-e4615cad468e/agents");
        let topic_exp = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: "v1".to_string(),
            room_id: Some(Uuid::parse_str("058df470-73ea-43a4-b36c-e4615cad468e").unwrap()),
            resource: Some(Resource {
                kind: ResourceKind::Agents,
                id: None,
            }),
        });
        assert_eq!(topic, Done("", topic_exp));

        let topic = agent_topic("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1/rooms/058df470-73ea-43a4-b36c-e4615cad468e/agents/");
        let topic_exp = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: "v1".to_string(),
            room_id: Some(Uuid::parse_str("058df470-73ea-43a4-b36c-e4615cad468e").unwrap()),
            resource: Some(Resource {
                kind: ResourceKind::Agents,
                id: None,
            }),
        });
        assert_eq!(topic, Done("/", topic_exp));

        let topic = agent_topic("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1/rooms/058df470-73ea-43a4-b36c-e4615cad468e/agents/c6d2eec6-94ac-4575-9658-10c93b939d9a");
        let topic_exp = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: "v1".to_string(),
            room_id: Some(Uuid::parse_str("058df470-73ea-43a4-b36c-e4615cad468e").unwrap()),
            resource: Some(Resource {
                kind: ResourceKind::Agents,
                id: Some(Uuid::parse_str("c6d2eec6-94ac-4575-9658-10c93b939d9a").unwrap()),
            }),
        });
        assert_eq!(topic, Done("", topic_exp));
    }

    #[test]
    fn parse_ping_topic() {
        let topic = ping_topic("ping");
        assert_eq!(topic, Done("", Topic::Ping(PingTopicKind::Ping)));
    }

    #[test]
    fn parse_pong_topic() {
        let topic = ping_topic("pong");
        assert_eq!(topic, Done("", Topic::Ping(PingTopicKind::Pong)));
    }

    #[test]
    fn parse_topic() {
        let topic = Topic::parse("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1/rooms/058df470-73ea-43a4-b36c-e4615cad468e/agents/c6d2eec6-94ac-4575-9658-10c93b939d9a");
        assert!(topic.is_ok());

        if let Topic::Agent(t) = topic.unwrap() {
            assert_eq!(t.kind, AgentTopicKind::Out);
            assert_eq!(
                t.agent_id,
                Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap()
            );
            assert_eq!(t.version, "v1");
            assert_eq!(
                t.room_id,
                Some(Uuid::parse_str("058df470-73ea-43a4-b36c-e4615cad468e").unwrap())
            );
            assert_eq!(
                t.resource,
                Some(Resource {
                    kind: ResourceKind::Agents,
                    id: Some(Uuid::parse_str("c6d2eec6-94ac-4575-9658-10c93b939d9a").unwrap()),
                })
            )
        } else {
            assert!(false);
        }

        let topic = Topic::parse("ping");
        assert!(topic.is_ok());
        assert_eq!(topic.unwrap(), Topic::Ping(PingTopicKind::Ping));

        let topic = Topic::parse("pong");
        assert!(topic.is_ok());
        assert_eq!(topic.unwrap(), Topic::Ping(PingTopicKind::Pong));

        let topic = Topic::parse("foo");
        assert!(topic.is_err());
    }

    #[test]
    fn get_reverse_ping_topic() {
        let ping_topic = PingTopicKind::Ping;
        let pong_topic = ping_topic.get_reverse();

        assert_eq!(pong_topic, PingTopicKind::Pong);
    }

    #[test]
    fn get_reverse_agent_topic() {
        let out_topic = AgentTopic {
            kind: AgentTopicKind::Out,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: "v1".to_string(),
            room_id: None,
            resource: Some(Resource {
                kind: ResourceKind::Agents,
                id: Some(Uuid::parse_str("c6d2eec6-94ac-4575-9658-10c93b939d9a").unwrap()),
            }),
        };
        let in_topic = out_topic.get_reverse();

        let expected = AgentTopic {
            kind: AgentTopicKind::In,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: "v1".to_string(),
            room_id: None,
            resource: Some(Resource {
                kind: ResourceKind::Agents,
                id: Some(Uuid::parse_str("c6d2eec6-94ac-4575-9658-10c93b939d9a").unwrap()),
            }),
        };

        assert_eq!(in_topic, expected);
    }

    #[test]
    fn parse_uuid() {
        let uuid_str = "7a648f41-bf0a-40cb-b844-16a58f0bff11";
        let expected_uuid = Uuid::parse_str(uuid_str).unwrap();
        assert_eq!(uuid(uuid_str), Done("", expected_uuid));

        use nom::ErrorKind::Verify;
        use nom::IResult::Error;

        let uuid_str = "7a648f41-bf0a-40cb-b844-16a58f0bff1z";
        assert_eq!(uuid(uuid_str), Error(Verify));
    }

    #[test]
    fn display_topic() {
        let topic = Topic::Ping(PingTopicKind::Ping);
        assert_eq!(topic.to_string(), "ping");

        let topic = Topic::Ping(PingTopicKind::Pong);
        assert_eq!(topic.to_string(), "pong");

        let topic = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: "v1".to_string(),
            room_id: Some(Uuid::parse_str("058df470-73ea-43a4-b36c-e4615cad468e").unwrap()),
            resource: Some(Resource {
                kind: ResourceKind::Agents,
                id: Some(Uuid::parse_str("c6d2eec6-94ac-4575-9658-10c93b939d9a").unwrap()),
            }),
        });
        let expected = "agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1/rooms/058df470-73ea-43a4-b36c-e4615cad468e/agents/c6d2eec6-94ac-4575-9658-10c93b939d9a";
        assert_eq!(topic.to_string(), expected);
    }
}
