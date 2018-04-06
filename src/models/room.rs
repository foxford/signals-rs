use chrono::NaiveDateTime;
use uuid::Uuid;

use messages::room::CreateRequest;
use schema::room;

#[derive(Queryable, Debug)]
pub struct Room {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub capacity: i16,
}

#[derive(Insertable, Debug)]
#[table_name = "room"]
pub struct NewRoom {
    pub capacity: i16,
}

impl From<CreateRequest> for NewRoom {
    fn from(req: CreateRequest) -> Self {
        let data = req.data;

        NewRoom {
            capacity: data.capacity,
        }
    }
}
