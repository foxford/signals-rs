use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Queryable, Debug)]
pub struct Room {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
}
