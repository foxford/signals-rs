use chrono::NaiveDateTime;
use uuid::Uuid;

use models::{Agent, Room};
use schema::room_agent;

#[derive(Associations, Identifiable, Queryable, Debug)]
#[table_name = "room_agent"]
#[primary_key(agent_id, room_id)]
#[belongs_to(Agent)]
#[belongs_to(Room)]
pub struct RoomAgent {
    pub agent_id: Uuid,
    pub room_id: Uuid,
    pub label: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "room_agent"]
pub struct NewRoomAgent {
    pub agent_id: Uuid,
    pub room_id: Uuid,
    pub label: String,
}
