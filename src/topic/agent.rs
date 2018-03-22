use nom::types::CompleteStr;
use uuid::Uuid;

use std::fmt;
use std::str::FromStr;

use topic::{Reversible, Topic};
use version::Version;

named!(pub topic<CompleteStr, Topic>,
    do_parse!(
        tag_s!("agents/") >>
        agent_id: map_res!(take_until_s!("/"), |s: CompleteStr| FromStr::from_str(s.0)) >>
        tag_s!("/") >>
        kind: alt!(map!(tag_s!("in"), |_| AgentTopicKind::In) | map!(tag_s!("out"), |_| AgentTopicKind::Out)) >>
        tag_s!("/signals.netology-group.services/api/") >>
        version: map!(tag_s!("v1"), |_| Version::V1) >>
        opt!(tag_s!("/")) >>
        eof!() >>

        (Topic::Agent(AgentTopic { kind, agent_id, version }))
    )
);

#[derive(Debug, PartialEq)]
pub struct AgentTopic {
    pub kind: AgentTopicKind,
    pub agent_id: Uuid,
    pub version: Version,
}

impl AgentTopic {
    pub fn new_in(agent_id: Uuid) -> AgentTopic {
        AgentTopic {
            kind: AgentTopicKind::In,
            agent_id,
            version: Version::V1,
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
            version: self.version,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AgentTopicKind {
    In,
    Out,
}

impl fmt::Display for AgentTopicKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = format!("{:?}", self).to_lowercase();
        f.write_str(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Err::Error as NomError;
    use nom::ErrorKind::*;
    use nom::types::CompleteStr;

    #[test]
    fn parse_topic() {
        let t = topic(CompleteStr("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1"));
        let topic_exp = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: Version::V1,
        });
        assert_eq!(t, Ok((CompleteStr(""), topic_exp)));

        let t = topic(CompleteStr("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1/"));
        let topic_exp = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: Version::V1,
        });
        assert_eq!(t, Ok((CompleteStr(""), topic_exp)));
    }

    #[test]
    fn parse_topic_with_wrong_uuid() {
        let topic = topic(CompleteStr(
            "agents/e19c94cf-53eb-4048-9c94-7ae74ff6d91/out/signals.netology-group.services/api/v1",
        ));

        assert_eq!(topic, Err(NomError(error_position!(CompleteStr("e19c94cf-53eb-4048-9c94-7ae74ff6d91/out/signals.netology-group.services/api/v1"), MapRes))));
    }

    #[test]
    fn parse_topic_with_extra_words() {
        let topic = topic(CompleteStr("agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1/rooms"));
        assert_eq!(
            topic,
            Err(NomError(error_position!(CompleteStr("rooms"), Eof)))
        );
    }

    #[test]
    fn get_reverse_topic() {
        let out_topic = AgentTopic {
            kind: AgentTopicKind::Out,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: Version::V1,
        };

        let expected = AgentTopic {
            kind: AgentTopicKind::In,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: Version::V1,
        };

        assert_eq!(out_topic.get_reverse(), expected);
    }

    #[test]
    fn display_topic() {
        let topic = Topic::Agent(AgentTopic {
            kind: AgentTopicKind::Out,
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: Version::V1,
        });
        let expected = "agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/out/signals.netology-group.services/api/v1";
        assert_eq!(topic.to_string(), expected);
    }
}
