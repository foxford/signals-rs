use uuid::Uuid;

use schema::agent;

#[derive(Associations, Identifiable, Queryable, Debug)]
#[table_name = "agent"]
pub struct Agent {
    pub id: Uuid,
}

#[derive(Insertable, Debug, PartialEq, Serialize, Deserialize)]
#[table_name = "agent"]
pub struct NewAgent {
    pub id: Uuid,
}
