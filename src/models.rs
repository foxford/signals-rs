use schema::{agents, rooms};
use uuid::Uuid;

#[derive(Identifiable, Queryable, Debug)]
pub struct Room {
    pub id: Uuid,
    pub label: String,
}

#[derive(AsChangeset, Insertable, Debug, PartialEq, Serialize, Deserialize)]
#[table_name = "rooms"]
pub struct NewRoom {
    pub label: Option<String>,
}

#[derive(Associations, Identifiable, Queryable, Debug)]
#[belongs_to(Room)]
pub struct Agent {
    pub id: Uuid,
    pub label: String,
    pub room_id: Uuid,
}

#[derive(AsChangeset, Insertable, Debug, PartialEq, Serialize, Deserialize)]
#[table_name = "agents"]
pub struct NewAgent {
    pub id: Uuid,
    pub label: Option<String>,
    pub room_id: Uuid,
}
