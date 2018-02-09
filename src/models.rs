use schema::rooms;
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
