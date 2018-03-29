use serde_json::Value;
use uuid::Uuid;

use messages::track::CreateRequest;
use models::Agent;
use schema::track;

#[derive(Associations, Identifiable, Queryable, Debug)]
#[table_name = "track"]
#[belongs_to(Agent, foreign_key = "owner_id")]
pub struct Track {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub metadata: Value,
}

#[derive(AsChangeset, Insertable, Debug, Deserialize)]
#[table_name = "track"]
pub struct NewTrack {
    pub owner_id: Uuid,
    pub metadata: Value,
}

impl From<CreateRequest> for NewTrack {
    fn from(req: CreateRequest) -> Self {
        let data = req.data;

        NewTrack {
            owner_id: data.owner_id,
            metadata: data.metadata,
        }
    }
}
