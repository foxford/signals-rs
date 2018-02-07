use schema::rooms;
use uuid::Uuid;

#[derive(Queryable, Debug)]
pub struct Room {
    pub id: Uuid,
    pub label: String,
}

#[derive(Insertable, Debug, PartialEq, Serialize, Deserialize)]
#[table_name = "rooms"]
pub struct NewRoom {
    pub label: Option<String>,
}
