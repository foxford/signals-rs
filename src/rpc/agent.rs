use diesel;
use diesel::prelude::*;

use errors::Result;
use models;
use rpc;
use schema::{agents, rooms};

use messages::agent::{CreateEvent, CreateRequest, CreateResponse, DeleteRequest, DeleteResponse,
                      ListRequest, ListResponse, ReadRequest, ReadResponse, UpdateRequest,
                      UpdateResponse};

build_rpc_trait! {
    pub trait Rpc {
        type Metadata;

        #[rpc(meta, name = "agent.create")]
        fn create(&self, Self::Metadata, CreateRequest) -> Result<CreateResponse>;

        #[rpc(name = "agent.read")]
        fn read(&self, ReadRequest) -> Result<ReadResponse>;

        #[rpc(name = "agent.update")]
        fn update(&self, UpdateRequest) -> Result<UpdateResponse>;

        #[rpc(name = "agent.delete")]
        fn delete(&self, DeleteRequest) -> Result<DeleteResponse>;

        #[rpc(name = "agent.list")]
        fn list(&self, ListRequest) -> Result<ListResponse>;
    }
}

pub struct RpcImpl;

impl Rpc for RpcImpl {
    type Metadata = rpc::Meta;

    fn create(&self, meta: rpc::Meta, req: CreateRequest) -> Result<CreateResponse> {
        println!("{:?}", meta);

        // FIXME: use connection pool
        let conn = establish_connection();

        let room: models::Room = rooms::table.find(req.room_id).first(&conn)?;

        let changeset = models::NewAgent {
            id: req.id,
            label: req.data.label.clone(),
            room_id: room.id,
        };

        let agent: models::Agent = diesel::insert_into(agents::table)
            .values(&changeset)
            .get_result(&conn)?;

        let resp = CreateResponse::new(&agent);

        let event = CreateEvent::new(room.id, resp.clone());
        let event_tx = meta.event_tx.unwrap();
        event_tx.send(event.into()).unwrap();

        Ok(resp)
    }

    fn read(&self, req: ReadRequest) -> Result<ReadResponse> {
        // FIXME: use connection pool
        let conn = establish_connection();

        let agent: models::Agent = agents::table
            .filter(agents::room_id.eq(req.room_id))
            .find(req.id)
            .first(&conn)?;

        Ok(ReadResponse::new(&agent))
    }

    fn update(&self, req: UpdateRequest) -> Result<UpdateResponse> {
        // FIXME: use connection pool
        let conn = establish_connection();

        let agent = agents::table
            .filter(agents::room_id.eq(req.room_id))
            .find(req.id);

        let agent: models::Agent = match req.data.label {
            Some(label) => diesel::update(agent)
                .set(agents::label.eq(label))
                .get_result(&conn)?,
            None => agent.first(&conn)?,
        };

        Ok(UpdateResponse::new(&agent))
    }

    fn delete(&self, req: DeleteRequest) -> Result<DeleteResponse> {
        // FIXME: use connection pool
        let conn = establish_connection();

        let agent = agents::table
            .filter(agents::room_id.eq(req.room_id))
            .find(req.id);

        let agent: models::Agent = diesel::delete(agent).get_result(&conn)?;

        Ok(DeleteResponse::new(&agent))
    }

    fn list(&self, req: ListRequest) -> Result<ListResponse> {
        // FIXME: use connection pool
        let conn = establish_connection();

        let room: models::Room = rooms::table.find(req.room_id).first(&conn)?;

        let agents = models::Agent::belonging_to(&room).load::<models::Agent>(&conn)?;

        Ok(ListResponse::new(&agents))
    }
}

fn establish_connection() -> PgConnection {
    let database_url = ::std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap()
}
