use nom::{alphanumeric, is_hex_digit};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

use errors::*;

named!(ping_topic<&str, Topic>,
    map!(tag_s!("ping"), |_| Topic::Ping)
);

named!(pong_topic<&str, Topic>,
    map!(tag_s!("pong"), |_| Topic::Pong)
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

        (Topic::Agent(AgentTopic { kind, agent_id, version, room_id }))
    )
);

named!(topic<&str, Topic>,
    alt!(ping_topic | pong_topic | agent_topic)
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
    Ping,
    Pong,
    Agent(AgentTopic),
}

impl Topic {
    pub fn parse(topic_str: &str) -> Result<Topic> {
        Ok(topic(topic_str).to_result()?)
    }
}

impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: String = match *self {
            Topic::Agent(ref t) => t.to_string(),
            _ => format!("{:?}", self).to_lowercase(),
        };

        f.write_str(&value)
    }
}

#[derive(Debug, PartialEq)]
pub struct AgentTopic {
    kind: AgentTopicKind,
    agent_id: Uuid,
    version: String,
    pub room_id: Option<Uuid>,
}

#[derive(Debug, PartialEq)]
enum AgentTopicKind {
    In,
    Out,
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
        f.write_str(&topic)
    }
}

impl fmt::Display for AgentTopicKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
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
        });
        assert_eq!(topic, Done("", topic_exp));

        let topic = agent_topic("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1/rooms/");
        let topic_exp = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: "v1".to_string(),
            room_id: None,
        });
        assert_eq!(topic, Done("/", topic_exp));

        let topic = agent_topic("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1/rooms/058df470-73ea-43a4-b36c-e4615cad468e");
        let topic_exp = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: "v1".to_string(),
            room_id: Some(Uuid::parse_str("058df470-73ea-43a4-b36c-e4615cad468e").unwrap()),
        });
        assert_eq!(topic, Done("", topic_exp));

        let topic =
            agent_topic("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals/api/v1/rooms/456");
        assert!(topic.is_err());
    }

    #[test]
    fn parse_ping_topic() {
        let topic = ping_topic("ping");
        assert_eq!(topic, Done("", Topic::Ping));
    }

    #[test]
    fn parse_pong_topic() {
        let topic = pong_topic("pong");
        assert_eq!(topic, Done("", Topic::Pong));
    }

    #[test]
    fn parse_topic() {
        let topic = Topic::parse("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1/rooms/058df470-73ea-43a4-b36c-e4615cad468e");
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
        } else {
            assert!(false);
        }

        let topic = Topic::parse("ping");
        assert!(topic.is_ok());
        assert_eq!(topic.unwrap(), Topic::Ping);

        let topic = Topic::parse("pong");
        assert!(topic.is_ok());
        assert_eq!(topic.unwrap(), Topic::Pong);

        let topic = Topic::parse("foo");
        assert!(topic.is_err());
    }

    #[test]
    fn get_reverse_topic() {
        let out_topic_str = "agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1/rooms";
        let out_topic = Topic::parse(out_topic_str).unwrap();

        match out_topic {
            Topic::Agent(t) => {
                let in_topic = t.get_reverse();
                let in_topic_str = "agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/in/signals.netology-group.services/api/v1/rooms";

                assert_eq!(in_topic_str, format!("{}", in_topic));
            }
            _ => assert!(false),
        }
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
        let topic = Topic::Ping;
        assert_eq!(topic.to_string(), "ping");

        let topic = Topic::Pong;
        assert_eq!(topic.to_string(), "pong");

        let topic = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: "v1".to_string(),
            room_id: Some(Uuid::parse_str("058df470-73ea-43a4-b36c-e4615cad468e").unwrap()),
        });
        let expected = "agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1/rooms/058df470-73ea-43a4-b36c-e4615cad468e";
        assert_eq!(topic.to_string(), expected);
    }
}
