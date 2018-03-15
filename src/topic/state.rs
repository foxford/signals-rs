use nom::types::CompleteStr;
use uuid::Uuid;

use std::fmt;
use std::str::FromStr;

use topic::Topic;
use version::Version;

named!(pub topic<CompleteStr, Topic>,
    do_parse!(
        tag_s!("agents/") >>
        agent_id: map_res!(take_until_s!("/"), |s: CompleteStr| FromStr::from_str(s.0)) >>
        tag_s!("/state/api/") >>
        version: map!(tag_s!("v1"), |_| Version::V1) >>
        eof!() >>

        (Topic::State(StateTopic { agent_id, version }))
    )
);

#[derive(Debug, PartialEq)]
pub struct StateTopic {
    pub agent_id: Uuid,
    pub version: Version,
}

impl StateTopic {
    pub fn new(agent_id: Uuid) -> StateTopic {
        StateTopic {
            agent_id,
            version: Version::V1,
        }
    }
}

impl fmt::Display for StateTopic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "agents/{}/state/api/{}", self.agent_id, self.version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use topic::Topic;

    #[test]
    fn parse_topic() {
        let t = topic(CompleteStr(
            "agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/state/api/v1",
        ));
        let topic_exp = Topic::State(StateTopic {
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: Version::V1,
        });
        assert_eq!(t, Ok((CompleteStr(""), topic_exp)));
    }

    #[test]
    fn display_topic() {
        let topic = Topic::State(StateTopic {
            agent_id: Uuid::parse_str("e19c94cf-53eb-4048-9c94-7ae74ff6d912").unwrap(),
            version: Version::V1,
        });
        let expected = "agents/e19c94cf-53eb-4048-9c94-7ae74ff6d912/state/api/v1";
        assert_eq!(topic.to_string(), expected);
    }
}
