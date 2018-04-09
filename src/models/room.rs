use chrono::NaiveDateTime;
use uuid::Uuid;

use std::fmt;

use messages::room::CreateRequest;
use schema::room;

#[derive(Queryable, Debug)]
pub struct Room {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub capacity: RoomCapacity,
}

#[derive(Insertable, Debug)]
#[table_name = "room"]
pub struct NewRoom {
    pub capacity: RoomCapacity,
}

impl From<CreateRequest> for NewRoom {
    fn from(req: CreateRequest) -> Self {
        let data = req.data;

        NewRoom {
            capacity: data.capacity,
        }
    }
}

#[derive(AsExpression, FromSqlRow, Clone, Copy, Debug, Serialize, Deserialize)]
#[sql_type = "SmallInt"]
pub struct RoomCapacity(u8);

impl RoomCapacity {
    pub fn new(cap: u8) -> RoomCapacity {
        RoomCapacity(cap)
    }
}

impl Default for RoomCapacity {
    fn default() -> Self {
        RoomCapacity(0)
    }
}

impl fmt::Display for RoomCapacity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

use std::cmp;
use std::cmp::Ordering;

impl cmp::PartialEq for RoomCapacity {
    fn eq(&self, other: &RoomCapacity) -> bool {
        self.0 == other.0
    }
}

impl cmp::PartialEq<u8> for RoomCapacity {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl cmp::PartialOrd for RoomCapacity {
    fn partial_cmp(&self, other: &RoomCapacity) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl cmp::PartialOrd<u8> for RoomCapacity {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        Some(self.0.cmp(&other))
    }
}

impl cmp::PartialEq<RoomCapacity> for u8 {
    fn eq(&self, other: &RoomCapacity) -> bool {
        *self == other.0
    }
}

impl cmp::PartialOrd<RoomCapacity> for u8 {
    fn partial_cmp(&self, other: &RoomCapacity) -> Option<Ordering> {
        Some(self.cmp(&other.0))
    }
}

use diesel::serialize::{self, Output, ToSql};
use std::io::Write;

impl ToSql<SmallInt, Pg> for RoomCapacity {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        let x = i16::from(self.0);
        // i16::to_sql(&x, out)
        ToSql::<SmallInt, Pg>::to_sql(&x, out)
    }
}

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::sql_types::{SmallInt};

impl FromSql<SmallInt, Pg> for RoomCapacity {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        // let x = i16::from_sql(bytes)?;
        let x = <i16 as FromSql<SmallInt, Pg>>::from_sql(bytes)?;
        Ok(RoomCapacity(x as u8))
    }
}
