use chrono::NaiveDateTime;
use uuid::Uuid;

use schema::{agent, local_track, remote_track};

#[derive(Queryable, Debug)]
pub struct Room {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Associations, Identifiable, Queryable, Debug)]
#[table_name = "agent"]
#[belongs_to(Room)]
pub struct Agent {
    pub id: Uuid,
    pub label: String,
    pub room_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(AsChangeset, Insertable, Debug, PartialEq, Serialize, Deserialize)]
#[table_name = "agent"]
pub struct NewAgent {
    pub id: Uuid,
    pub label: String,
    pub room_id: Uuid,
}

#[derive(Associations, Identifiable, Queryable, Debug)]
#[table_name = "local_track"]
#[belongs_to(Agent, foreign_key = "owner_id")]
pub struct LocalTrack {
    pub id: Uuid,
    pub stream_id: String,
    pub track_id: String,
    pub device: String,
    pub kind: String,
    pub label: String,
    pub owner_id: Uuid,
}

#[derive(AsChangeset, Insertable, Debug, Deserialize)]
#[table_name = "local_track"]
pub struct NewLocalTrack {
    pub stream_id: String,
    pub track_id: String,
    pub device: String,
    pub kind: String,
    pub label: String,
    pub owner_id: Uuid,
}

#[derive(Associations, Identifiable, Queryable, Debug)]
#[table_name = "remote_track"]
#[belongs_to(LocalTrack)]
pub struct RemoteTrack {
    pub id: Uuid,
    pub local_track_id: Uuid,
    pub agent_id: Uuid,
}

#[derive(AsChangeset, Insertable, Debug, Deserialize)]
#[table_name = "remote_track"]
pub struct NewRemoteTrack {
    pub local_track_id: Uuid,
    pub agent_id: Uuid,
}
