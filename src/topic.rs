use nom::alphanumeric;
use std::str::FromStr;

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
        agent: map_res!(alphanumeric, FromStr::from_str) >>
        tag_s!("/") >>
        kind: alt!(map!(tag_s!("in"), |_| AgentTopicKind::In) | map!(tag_s!("out"), |_| AgentTopicKind::Out)) >>
        tag_s!("/signals.netology-group.services/api/") >>
        version: map_res!(alphanumeric, FromStr::from_str) >>
        tag_s!("/rooms") >>
        room: opt!(complete!(preceded!(
            tag_s!("/"),
            map_res!(alphanumeric, FromStr::from_str)
        ))) >>

        (Topic::Agent(AgentTopic { kind, agent, version, room }))
    )
);

named!(topic<&str, Topic>,
    alt!(ping_topic | pong_topic | agent_topic)
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

#[derive(Debug, PartialEq)]
pub struct AgentTopic {
    kind: AgentTopicKind,
    agent: String,
    version: String,
    pub room: Option<String>,
}

#[derive(Debug, PartialEq)]
enum AgentTopicKind {
    In,
    Out,
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::Done;

    #[test]
    fn parse_agent_topic() {
        let topic = agent_topic("agents/123/out/signals.netology-group.services/api/v1/rooms");
        let topic_exp = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent: "123".to_string(),
            version: "v1".to_string(),
            room: None,
        });
        assert_eq!(topic, Done("", topic_exp));

        let topic = agent_topic("agents/123/out/signals.netology-group.services/api/v1/rooms/");
        let topic_exp = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent: "123".to_string(),
            version: "v1".to_string(),
            room: None,
        });
        assert_eq!(topic, Done("/", topic_exp));

        let topic = agent_topic("agents/123/out/signals.netology-group.services/api/v1/rooms/456");
        let topic_exp = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent: "123".to_string(),
            version: "v1".to_string(),
            room: Some("456".to_string()),
        });
        assert_eq!(topic, Done("", topic_exp));

        let topic = agent_topic("agents/123/out/signals/api/v1/rooms/456");
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
        let topic = Topic::parse("agents/123/out/signals.netology-group.services/api/v1/rooms/456");
        assert!(topic.is_ok());

        if let Topic::Agent(t) = topic.unwrap() {
            assert_eq!(t.kind, AgentTopicKind::Out);
            assert_eq!(t.agent, "123");
            assert_eq!(t.version, "v1");
            assert_eq!(t.room, Some("456".to_string()));
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
}
