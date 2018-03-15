use nom::types::CompleteStr;

use std::fmt;

use topic::{Reversible, Topic};

named!(pub topic<CompleteStr, Topic>,
    alt!(
        map!(tag_s!("ping"), |_| Topic::Ping(PingTopicKind::Ping)) |
        map!(tag_s!("pong"), |_| Topic::Ping(PingTopicKind::Pong))
    )
);

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

impl Reversible for PingTopicKind {
    type Topic = PingTopicKind;

    fn get_reverse(&self) -> PingTopicKind {
        match *self {
            PingTopicKind::Ping => PingTopicKind::Pong,
            PingTopicKind::Pong => PingTopicKind::Ping,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ping_topic() {
        let topic = topic(CompleteStr("ping"));
        assert_eq!(
            topic,
            Ok((CompleteStr(""), Topic::Ping(PingTopicKind::Ping)))
        );
    }

    #[test]
    fn parse_pong_topic() {
        let topic = topic(CompleteStr("pong"));
        assert_eq!(
            topic,
            Ok((CompleteStr(""), Topic::Ping(PingTopicKind::Pong)))
        );
    }

    #[test]
    fn get_reverse_topic() {
        let ping_topic = PingTopicKind::Ping;
        let pong_topic = ping_topic.get_reverse();

        assert_eq!(pong_topic, PingTopicKind::Pong);
    }

    #[test]
    fn display_topic() {
        let topic = Topic::Ping(PingTopicKind::Ping);
        assert_eq!(topic.to_string(), "ping");

        let topic = Topic::Ping(PingTopicKind::Pong);
        assert_eq!(topic.to_string(), "pong");
    }
}
