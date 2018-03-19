use uuid::Uuid;

use models::Agent;
use schema::local_track;

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
