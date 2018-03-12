use schema::{agent, room};
use uuid::Uuid;

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "room"]
pub struct Room {
    pub id: Uuid,
    pub label: String,
}

#[derive(AsChangeset, Insertable, Debug, PartialEq, Serialize, Deserialize)]
#[table_name = "room"]
pub struct NewRoom {
    pub label: String,
}

#[derive(Associations, Identifiable, Queryable, Debug)]
#[table_name = "agent"]
#[belongs_to(Room)]
pub struct Agent {
    pub id: Uuid,
    pub label: String,
    pub room_id: Uuid,
}

#[derive(AsChangeset, Insertable, Debug, PartialEq, Serialize, Deserialize)]
#[table_name = "agent"]
pub struct NewAgent {
    pub id: Uuid,
    pub label: String,
    pub room_id: Uuid,
}
