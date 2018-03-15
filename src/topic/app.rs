use serde::{Serialize, Serializer};
use uuid::Uuid;

use std::fmt;

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    use topic::Topic;

    #[test]
    fn display_topic() {
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
    fn serialize_topic() {
        let topic = AppTopic {
            room_id: Uuid::parse_str("050b7c6f-795c-4cb4-aeea-5ee3f9083de2").unwrap(),
            resource: ResourceKind::Agents,
        };

        let expected = r#""apps/signals.netology-group.services/api/v1/rooms/050b7c6f-795c-4cb4-aeea-5ee3f9083de2/agents""#;
        assert_eq!(serde_json::to_string(&topic).unwrap(), expected);
    }
}
