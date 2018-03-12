use diesel;
use diesel::prelude::*;

use errors::Result;
use models;
use rpc;
use schema::room;

use messages::room::{CreateRequest, CreateResponse, DeleteRequest, DeleteResponse, ListResponse,
                     ReadRequest, ReadResponse, UpdateRequest, UpdateResponse};

build_rpc_trait! {
    pub trait Rpc {
        type Metadata;

        #[rpc(meta, name = "room.create")]
        fn create(&self, Self::Metadata, CreateRequest) -> Result<CreateResponse>;

        #[rpc(meta, name = "room.read")]
        fn read(&self, Self::Metadata, ReadRequest) -> Result<ReadResponse>;

        #[rpc(meta, name = "room.update")]
        fn update(&self, Self::Metadata, UpdateRequest) -> Result<UpdateResponse>;

        #[rpc(meta, name = "room.delete")]
        fn delete(&self, Self::Metadata, DeleteRequest) -> Result<DeleteResponse>;

        #[rpc(meta, name = "room.list")]
        fn list(&self, Self::Metadata) -> Result<ListResponse>;
    }
}

pub struct RpcImpl;

impl Rpc for RpcImpl {
    type Metadata = rpc::Meta;

    fn create(&self, meta: rpc::Meta, req: CreateRequest) -> Result<CreateResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let room: models::Room = diesel::insert_into(room::table)
            .values(&req.data)
            .get_result(conn)?;

        Ok(CreateResponse::new(&room))
    }

    fn read(&self, meta: rpc::Meta, req: ReadRequest) -> Result<ReadResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let room: models::Room = room::table.find(req.room_id).first(conn)?;

        Ok(ReadResponse::new(&room))
    }

    fn update(&self, meta: rpc::Meta, req: UpdateRequest) -> Result<UpdateResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let room = room::table.find(req.room_id);
        let room: models::Room = diesel::update(room).set(&req.data).get_result(conn)?;

        Ok(UpdateResponse::new(&room))
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
