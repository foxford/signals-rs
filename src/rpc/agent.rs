use std::str::FromStr;

use diesel;
use diesel::prelude::*;

use errors::{ErrorKind, Result};
use models;
use rpc;
use schema::{agents, rooms};

use messages::agent::{CreateEvent, CreateRequest, CreateResponse, DeleteEvent, DeleteRequest,
                      DeleteResponse, ListRequest, ListResponse, ReadRequest, ReadResponse,
                      UpdateRequest, UpdateResponse};
use messages::query_parameters;

build_rpc_trait! {
    pub trait Rpc {
        type Metadata;

        #[rpc(meta, name = "agent.create")]
        fn create(&self, Self::Metadata, CreateRequest) -> Result<CreateResponse>;

        #[rpc(meta, name = "agent.read")]
        fn read(&self, Self::Metadata, ReadRequest) -> Result<ReadResponse>;

        #[rpc(meta, name = "agent.update")]
        fn update(&self, Self::Metadata, UpdateRequest) -> Result<UpdateResponse>;

        #[rpc(meta, name = "agent.delete")]
        fn delete(&self, Self::Metadata, DeleteRequest) -> Result<DeleteResponse>;

        #[rpc(meta, name = "agent.list")]
        fn list(&self, Self::Metadata, ListRequest) -> Result<ListResponse>;
    }
}

pub struct RpcImpl;

impl Rpc for RpcImpl {
    type Metadata = rpc::Meta;

    fn create(&self, meta: rpc::Meta, req: CreateRequest) -> Result<CreateResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let room: models::Room = rooms::table.find(req.room_id).first(conn)?;

        let changeset = models::NewAgent {
            id: req.id,
            label: req.data.label.clone(),
            room_id: room.id,
        };

        let agent: models::Agent = diesel::insert_into(agents::table)
            .values(&changeset)
            .get_result(conn)?;

        let resp = CreateResponse::new(&agent);

        let event = CreateEvent::new(room.id, resp.clone());
        let event_tx = meta.event_tx.unwrap();
        event_tx.send(event.into()).unwrap();

        Ok(resp)
    }

    fn read(&self, meta: rpc::Meta, req: ReadRequest) -> Result<ReadResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let agent: models::Agent = agents::table
            .filter(agents::room_id.eq(req.room_id))
            .find(req.id)
            .first(conn)?;

        Ok(ReadResponse::new(&agent))
    }

    fn update(&self, meta: rpc::Meta, req: UpdateRequest) -> Result<UpdateResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let agent = agents::table
            .filter(agents::room_id.eq(req.room_id))
            .find(req.id);

        let agent: models::Agent = diesel::update(agent)
            .set(agents::label.eq(req.data.label))
            .get_result(conn)?;

        Ok(UpdateResponse::new(&agent))
    }

    fn delete(&self, meta: rpc::Meta, req: DeleteRequest) -> Result<DeleteResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let agent = agents::table
            .filter(agents::room_id.eq(req.room_id))
            .find(req.id);

        let agent: models::Agent = diesel::delete(agent).get_result(conn)?;

        let resp = DeleteResponse::new(&agent);

        let event = DeleteEvent::new(req.room_id, resp.clone());
        let event_tx = meta.event_tx.unwrap();
        event_tx.send(event.into()).unwrap();

        Ok(resp)
    }

    fn list(&self, meta: rpc::Meta, req: ListRequest) -> Result<ListResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let mut query = agents::table.into_boxed();
        if let Some(fq) = req.fq {
            let expr = query_parameters::Expr::from_str(&fq)?;
            match expr {
                query_parameters::Expr::Value(filter) => match filter {
                    query_parameters::Filter::RoomId(id) => {
                        query = query.filter(agents::room_id.eq(id));
                    }
                    _ => Err(ErrorKind::BadRequest)?,
                },
                _ => Err(ErrorKind::BadRequest)?,
            }
        }

        let agents = query.load::<models::Agent>(conn)?;

        Ok(ListResponse::new(&agents))
    }
}
