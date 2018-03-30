use diesel;
use diesel::prelude::*;

use std::str::FromStr;

use messages::EventKind;
use messages::agent::{CreateRequest, CreateResponse, DeleteRequest, DeleteResponse, JoinEvent,
                      JoinEventPayload, JoinRequest, JoinResponse, LeaveEvent, LeaveRequest,
                      LeaveResponse, ListRequest, ListResponse, ReadRequest, ReadResponse,
                      UpdateRequest, UpdateResponse};
use messages::query_parameters;
use messages::track::{DeleteEvent as TrackDeleteEvent, DeleteResponse as TrackDeleteResponse};
use models;
use rpc;
use rpc::error::{Error, Result};
use schema::{agent, room, room_agent, track};

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

        #[rpc(meta, name = "agent.join_room")]
        fn join_room(&self, Self::Metadata, JoinRequest) -> Result<JoinResponse>;

        #[rpc(meta, name = "agent.leave_room")]
        fn leave_room(&self, Self::Metadata, LeaveRequest) -> Result<LeaveResponse>;
    }
}

pub struct RpcImpl;

impl Rpc for RpcImpl {
    type Metadata = rpc::Meta;

    fn create(&self, meta: rpc::Meta, req: CreateRequest) -> Result<CreateResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let changeset = models::NewAgent { id: req.id };
        let agent: models::Agent = diesel::insert_into(agent::table)
            .values(&changeset)
            .get_result(conn)?;

        let resp = CreateResponse::new(&agent);

        Ok(resp)
    }

    fn read(&self, meta: rpc::Meta, req: ReadRequest) -> Result<ReadResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let agent = room_agent::table
            .find((req.id, req.room_id))
            .first::<models::RoomAgent>(conn)?;

        Ok(ReadResponse::new(&agent))
    }

    fn update(&self, meta: rpc::Meta, req: UpdateRequest) -> Result<UpdateResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let agent = room_agent::table.find((req.id, req.room_id));

        let agent = diesel::update(agent)
            .set(room_agent::label.eq(req.data.label))
            .get_result::<models::RoomAgent>(conn)?;

        Ok(UpdateResponse::new(&agent))
    }

    fn delete(&self, meta: rpc::Meta, req: DeleteRequest) -> Result<DeleteResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let agent = agent::table.find(req.id).first::<models::Agent>(conn)?;

        let (room_agents, tracks) = conn.transaction::<_, Error, _>(|| {
            let room_agents = diesel::delete(
                room_agent::table.filter(room_agent::agent_id.eq(agent.id)),
            ).get_results::<models::RoomAgent>(conn)?;

            let tracks = diesel::delete(track::table.filter(track::owner_id.eq(agent.id)))
                .get_results::<models::Track>(conn)?;

            diesel::delete(&agent).execute(conn)?;

            Ok((room_agents, tracks))
        })?;

        let notification_tx = meta.notification_tx.unwrap();

        for room_agent in &room_agents {
            let room_id = room_agent.room_id;

            for track in &tracks {
                let payload = TrackDeleteResponse::new(track);
                let event = TrackDeleteEvent::new(room_id, payload);
                let event_kind = EventKind::from(event);
                notification_tx.send(event_kind.into()).unwrap();
            }

            let payload = LeaveResponse::new(room_agent);
            let event = LeaveEvent::new(room_id, payload);
            let event_kind = EventKind::from(event);
            notification_tx.send(event_kind.into()).unwrap();
        }

        let resp = DeleteResponse::new(&agent);

        Ok(resp)
    }

    fn list(&self, meta: rpc::Meta, req: ListRequest) -> Result<ListResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let mut query = room_agent::table.into_boxed();

        if let Some(fq) = req.fq {
            let expr = query_parameters::Expr::from_str(&fq)?;
            match expr {
                query_parameters::Expr::Value(filter) => match filter {
                    query_parameters::Filter::RoomId(id) => {
                        query = query.filter(room_agent::room_id.eq(id));
                    }
                    _ => Err(Error::BadRequest)?,
                },
                _ => Err(Error::BadRequest)?,
            }
        }

        let agents = query.load::<models::RoomAgent>(conn)?;

        Ok(ListResponse::new(&agents))
    }

    fn join_room(&self, meta: rpc::Meta, req: JoinRequest) -> Result<JoinResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let room: models::Room = room::table.find(req.room_id).first(conn)?;

        let changeset = models::NewRoomAgent {
            room_id: room.id,
            agent_id: req.id,
            label: req.data.label.clone(),
        };

        let agent: models::RoomAgent = diesel::insert_into(room_agent::table)
            .values(&changeset)
            .get_result(conn)?;

        let resp = JoinResponse::new(&agent);

        let payload = JoinEventPayload::new(agent.agent_id, agent.room_id);
        let event = JoinEvent::new(room.id, payload);
        let notification_tx = meta.notification_tx.unwrap();
        let event_kind = EventKind::from(event);
        notification_tx.send(event_kind.into()).unwrap();

        Ok(resp)
    }

    fn leave_room(&self, meta: rpc::Meta, req: LeaveRequest) -> Result<LeaveResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let room_agent = room_agent::table
            .find((req.id, req.room_id))
            .first::<models::RoomAgent>(conn)?;

        diesel::delete(&room_agent).execute(conn)?;

        let notification_tx = meta.notification_tx.unwrap();
        let resp = LeaveResponse::new(&room_agent);

        let event = LeaveEvent::new(req.room_id, resp.clone());
        let event_kind = EventKind::from(event);
        notification_tx.send(event_kind.into()).unwrap();

        Ok(resp)
    }
}
