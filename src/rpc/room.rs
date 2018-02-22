use diesel;
use diesel::prelude::*;

use errors::Result;
use models;
use schema::rooms;

use messages::room::{CreateRequest, CreateResponse, DeleteRequest, DeleteResponse, ListResponse,
                     ReadRequest, ReadResponse, UpdateRequest, UpdateResponse};

build_rpc_trait! {
    pub trait Rpc {
        #[rpc(name = "room.create")]
        fn create(&self, CreateRequest) -> Result<CreateResponse>;

        #[rpc(name = "room.read")]
        fn read(&self, ReadRequest) -> Result<ReadResponse>;

        #[rpc(name = "room.update")]
        fn update(&self, UpdateRequest) -> Result<UpdateResponse>;

        #[rpc(name = "room.delete")]
        fn delete(&self, DeleteRequest) -> Result<DeleteResponse>;

        #[rpc(name = "room.list")]
        fn list(&self) -> Result<ListResponse>;
    }
}

pub struct RpcImpl;

impl Rpc for RpcImpl {
    fn create(&self, req: CreateRequest) -> Result<CreateResponse> {
        // FIXME: use connection pool
        let conn = establish_connection();

        let room: models::Room = diesel::insert_into(rooms::table)
            .values(&req.data)
            .get_result(&conn)?;

        Ok(CreateResponse::new(&room))
    }

    fn read(&self, req: ReadRequest) -> Result<ReadResponse> {
        // FIXME: use connection pool
        let conn = establish_connection();

        let room: models::Room = rooms::table.find(req.room_id).first(&conn)?;

        Ok(ReadResponse::new(&room))
    }

    fn update(&self, req: UpdateRequest) -> Result<UpdateResponse> {
        // FIXME: use connection pool
        let conn = establish_connection();

        let room = rooms::table.find(req.room_id);
        let room: models::Room = diesel::update(room).set(&req.data).get_result(&conn)?;

        Ok(UpdateResponse::new(&room))
    }

    fn delete(&self, req: DeleteRequest) -> Result<DeleteResponse> {
        // FIXME: use connection pool
        let conn = establish_connection();

        let room = rooms::table.find(req.room_id);
        let room: models::Room = diesel::delete(room).get_result(&conn)?;

        Ok(DeleteResponse::new(&room))
    }

    fn list(&self) -> Result<ListResponse> {
        // FIXME: use connection pool
        let conn = establish_connection();

        let rooms = rooms::table.load::<models::Room>(&conn)?;

        Ok(ListResponse::new(&rooms))
    }
}

fn establish_connection() -> PgConnection {
    let database_url = ::std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap()
}
