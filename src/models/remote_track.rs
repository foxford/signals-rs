use uuid::Uuid;

use models::LocalTrack;
use schema::remote_track;

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
