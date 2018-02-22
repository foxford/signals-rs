use nom::alphanumeric;
use serde::{Serialize, Serializer};
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
        agent_id: map_res!(take_until_s!("/"), FromStr::from_str) >>
        tag_s!("/") >>
        kind: alt!(map!(tag_s!("in"), |_| AgentTopicKind::In) | map!(tag_s!("out"), |_| AgentTopicKind::Out)) >>
        tag_s!("/signals.netology-group.services/api/") >>
        version: map_res!(alphanumeric, FromStr::from_str) >>
        opt!(complete!(tag_s!("/"))) >>
        eof!() >>

        (Topic::Agent(AgentTopic { kind, agent_id, version }))
    )
);

named!(topic<&str, Topic>,
    alt!(ping_topic | agent_topic)
);

#[derive(Debug, PartialEq)]
pub enum Topic {
    Ping(PingTopicKind),
    Agent(AgentTopic),
    App(AppTopic),
}

impl Topic {
    pub fn parse(topic_str: &str) -> Result<Topic> {
        Ok(topic(topic_str).to_result()?)
    }

    pub fn get_reverse(&self) -> Topic {
        match *self {
            Topic::Ping(ref t) => Topic::Ping(t.get_reverse()),
            Topic::Agent(ref t) => Topic::Agent(t.get_reverse()),
            Topic::App(_) => unreachable!(),
        }
    }
}

impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: &fmt::Display = match *self {
            Topic::Ping(ref t) => t,
            Topic::Agent(ref t) => t,
            Topic::App(ref t) => t,
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
}

impl AgentTopic {
    pub fn get_reverse(&self) -> AgentTopic {
        let kind = match self.kind {
            AgentTopicKind::In => AgentTopicKind::Out,
            AgentTopicKind::Out => AgentTopicKind::In,
        };

        AgentTopic {
            kind,
            agent_id: self.agent_id,
            version: self.version.clone(),
        }
    }
}

impl fmt::Display for AgentTopic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "agents/{}/{}/signals.netology-group.services/api/{}",
            self.agent_id, self.kind, self.version
        )
    }
}

#[derive(Debug, PartialEq)]
enum AgentTopicKind {
    In,
    Out,
}

impl fmt::Display for AgentTopicKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = format!("{:?}", self).to_lowercase();
        f.write_str(&value)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResourceKind {
    Agents,
    Tracks,
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = format!("{:?}", self).to_lowercase();
        f.write_str(&value)
    }
}

#[derive(Debug, PartialEq)]
pub struct AppTopic {
    pub room_id: Uuid,
    pub resource: ResourceKind,
}

impl fmt::Display for AppTopic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "apps/signals.netology-group.services/api/v1/rooms/{}/{}",
            self.room_id, self.resource
        )
    }
}

impl Serialize for AppTopic {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
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
            agent_id: self.agent_id,
            version: self.version.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::ErrorKind::*;
    use nom::IResult::{Done, Error};
    use serde_json;

    #[test]
    fn parse_agent_topic() {
        let topic = agent_topic("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1");
        let topic_exp = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: "v1".to_string(),
        });
        assert_eq!(topic, Done("", topic_exp));

        let topic = agent_topic("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1/");
        let topic_exp = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: "v1".to_string(),
        });
        assert_eq!(topic, Done("", topic_exp));
    }

    #[test]
    fn parse_agent_topic_with_wrong_uuid() {
        let topic = agent_topic(
            "agents/e19c94cf-53eb-4048-9c94-7ae74ff6d91/out/signals.netology-group.services/api/v1",
        );
        assert_eq!(topic, Error(MapRes));
    }

    #[test]
    fn parse_agent_topic_with_extra_words() {
        let topic = agent_topic(
            "agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1/rooms",
        );
        assert_eq!(topic, Error(Eof));
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
        let topic = Topic::parse("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1");
        assert!(topic.is_ok());

        if let Topic::Agent(t) = topic.unwrap() {
            assert_eq!(t.kind, AgentTopicKind::Out);
            assert_eq!(
                t.agent_id,
                Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap()
            );
            assert_eq!(t.version, "v1");
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
        };

        let expected = AgentTopic {
            kind: AgentTopicKind::In,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: "v1".to_string(),
        };

        assert_eq!(out_topic.get_reverse(), expected);
    }

    #[test]
    fn display_ping_topic() {
        let topic = Topic::Ping(PingTopicKind::Ping);
        assert_eq!(topic.to_string(), "ping");

        let topic = Topic::Ping(PingTopicKind::Pong);
        assert_eq!(topic.to_string(), "pong");
    }

    #[test]
    fn display_agent_topic() {
        let topic = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: "v1".to_string(),
        });
        let expected = "agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1";
        assert_eq!(topic.to_string(), expected);
    }

    #[test]
    fn display_app_topic() {
        let topic = Topic::App(AppTopic {
            room_id: Uuid::parse_str("058df470-73ea-43a4-b36c-e4615cad468e").unwrap(),
            resource: ResourceKind::Agents,
        });
        let expected = "apps/signals.netology-group.services/api/v1/rooms/058df470-73ea-43a4-b36c-e4615cad468e/agents";
        assert_eq!(topic.to_string(), expected);
    }

    #[test]
    fn serialize_resource_kind() {
        assert_eq!(
            serde_json::to_string(&ResourceKind::Agents).unwrap(),
            r#""agents""#
        );
    }

    #[test]
    fn serialize_app_topic() {
        let topic = AppTopic {
            room_id: Uuid::parse_str("050b7c6f-795c-4cb4-aeea-5ee3f9083de2").unwrap(),
            resource: ResourceKind::Agents,
        };

        let expected = r#""apps/signals.netology-group.services/api/v1/rooms/050b7c6f-795c-4cb4-aeea-5ee3f9083de2/agents""#;
        assert_eq!(serde_json::to_string(&topic).unwrap(), expected);
    }
}
