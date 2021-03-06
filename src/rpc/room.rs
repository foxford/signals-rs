use diesel;
use diesel::prelude::*;

use models;
use rpc;
use rpc::error::Result;
use schema::room;

use messages::room::{CreateResponse, DeleteRequest, DeleteResponse, ListResponse, ReadRequest,
                     ReadResponse};

build_rpc_trait! {
    pub trait Rpc {
        type Metadata;

        #[rpc(meta, name = "room.create")]
        fn create(&self, Self::Metadata) -> Result<CreateResponse>;

        #[rpc(meta, name = "room.read")]
        fn read(&self, Self::Metadata, ReadRequest) -> Result<ReadResponse>;

        #[rpc(meta, name = "room.delete")]
        fn delete(&self, Self::Metadata, DeleteRequest) -> Result<DeleteResponse>;

        #[rpc(meta, name = "room.list")]
        fn list(&self, Self::Metadata) -> Result<ListResponse>;
    }
}

pub struct RpcImpl;

impl Rpc for RpcImpl {
    type Metadata = rpc::Meta;

    fn create(&self, meta: rpc::Meta) -> Result<CreateResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let room: models::Room = diesel::insert_into(room::table)
            .default_values()
            .get_result(conn)?;

        Ok(CreateResponse::new(&room))
    }

    fn read(&self, meta: rpc::Meta, req: ReadRequest) -> Result<ReadResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let room: models::Room = room::table.find(req.room_id).first(conn)?;

        Ok(ReadResponse::new(&room))
    }

    fn delete(&self, meta: rpc::Meta, req: DeleteRequest) -> Result<DeleteResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let room = room::table.find(req.room_id);
        let room: models::Room = diesel::delete(room).get_result(conn)?;

        Ok(DeleteResponse::new(&room))
    }

    fn list(&self, meta: rpc::Meta) -> Result<ListResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let rooms = room::table.load::<models::Room>(conn)?;

        Ok(ListResponse::new(&rooms))
    }
}
