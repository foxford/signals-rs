use chrono::NaiveDateTime;
use uuid::Uuid;

use models::Room;
use schema::agent;

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
