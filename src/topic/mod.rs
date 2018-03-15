use nom::types::CompleteStr;

use std::fmt;

use error;

mod agent;
mod app;
mod ping;
mod state;

pub use topic::agent::AgentTopic;
use topic::agent::topic as agent_topic;
pub use topic::app::{AppTopic, ResourceKind};
use topic::ping::{topic as ping_topic, PingTopicKind};
pub use topic::state::{topic as state_topic, StateTopic};

named!(topic<CompleteStr, Topic>,
    alt!(ping_topic | agent_topic | state_topic)
);

#[derive(Debug, PartialEq)]
pub enum Topic {
    Ping(PingTopicKind),
    Agent(AgentTopic),
    App(AppTopic),
    State(StateTopic),
}

impl Topic {
    pub fn parse(topic_str: &str) -> Result<Topic, error::ParseError> {
        let (_, t) = topic(CompleteStr(topic_str))?;
        Ok(t)
    }

    pub fn get_reverse(&self) -> Option<Topic> {
        match *self {
            Topic::Ping(ref t) => Some(Topic::Ping(t.get_reverse())),
            Topic::Agent(ref t) => Some(Topic::Agent(t.get_reverse())),
            Topic::App(_) | Topic::State(_) => None,
        }
    }
}

impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: &fmt::Display = match *self {
            Topic::Ping(ref t) => t,
            Topic::Agent(ref t) => t,
            Topic::App(ref t) => t,
            Topic::State(ref t) => t,
        };

        write!(f, "{}", value)
    }
}

pub trait Reversible {
    type Topic;

    fn get_reverse(&self) -> Self::Topic;
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn parse_ping_topic() {
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
    fn parse_agent_topic() {
        use topic::agent::AgentTopicKind;
        use version::Version;

        let topic = Topic::parse("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1");
        assert!(topic.is_ok());

        if let Topic::Agent(t) = topic.unwrap() {
            assert_eq!(t.kind, AgentTopicKind::Out);
            assert_eq!(
                t.agent_id,
                Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap()
            );
            assert_eq!(t.version, Version::V1);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn parse_state_topic() {
        use version::Version;

        let topic = Topic::parse("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/state/api/v1");
        assert!(topic.is_ok());

        if let Topic::State(t) = topic.unwrap() {
            assert_eq!(
                t.agent_id,
                Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap()
            );
            assert_eq!(t.version, Version::V1);
        } else {
            assert!(false);
        }
    }
}
