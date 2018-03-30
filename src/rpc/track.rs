use diesel::prelude::*;
use diesel::{self, PgConnection};
use uuid::Uuid;

use std::str::FromStr;

use messages::EventKind;
use messages::query_parameters;
use messages::track::{CreateEvent, CreateRequest, CreateResponse, DeleteEvent, DeleteRequest,
                      DeleteResponse, ListRequest, ListResponse};
use models;
use rpc;
use rpc::error::{Error, Result};
use schema::{agent, room_agent, track};

macro_rules! and_filter {
    ($query:ident, $filter:ident) => {
        match $filter {
            query_parameters::Filter::RoomId(id) => {
                $query = $query.filter(room_agent::room_id.eq(id));
            }
            query_parameters::Filter::OwnerId(id) => {
                $query = $query.filter(track::owner_id.eq(id));
            }
        }
    };
}

build_rpc_trait! {
    pub trait Rpc {
        type Metadata;

        #[rpc(meta, name = "track.create")]
        fn create(&self, Self::Metadata, CreateRequest) -> Result<CreateResponse>;

        #[rpc(meta, name = "track.delete")]
        fn delete(&self, Self::Metadata, DeleteRequest) -> Result<DeleteResponse>;

        #[rpc(meta, name = "track.list")]
        fn list(&self, Self::Metadata, ListRequest) -> Result<ListResponse>;
    }
}

pub struct RpcImpl;

impl Rpc for RpcImpl {
    type Metadata = rpc::Meta;

    fn create(&self, meta: rpc::Meta, req: CreateRequest) -> Result<CreateResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let agent_id = req.data.owner_id;
        let changeset = models::NewTrack::from(req);

        let track: models::Track = diesel::insert_into(track::table)
            .values(&changeset)
            .get_result(conn)?;

        let resp = CreateResponse::new(&track);

        let notification_tx = meta.notification_tx.unwrap();
        let room_ids = get_agent_room_ids(conn, agent_id)?;
        for room_id in room_ids {
            let event = CreateEvent::new(room_id, resp.clone());
            let event_kind = EventKind::from(event);
            notification_tx.send(event_kind.into()).unwrap();
        }

        Ok(resp)
    }

    fn delete(&self, meta: rpc::Meta, req: DeleteRequest) -> Result<DeleteResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let target = track::table.find(req.id);
        let track = diesel::delete(target).get_result(conn)?;

        let resp = DeleteResponse::new(&track);

        let notification_tx = meta.notification_tx.unwrap();
        let room_ids = get_agent_room_ids(conn, track.owner_id)?;
        for room_id in room_ids {
            let event = DeleteEvent::new(room_id, resp.clone());
            let event_kind = EventKind::from(event);
            notification_tx.send(event_kind.into()).unwrap();
        }

        Ok(resp)
    }

    fn list(&self, meta: rpc::Meta, req: ListRequest) -> Result<ListResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let mut query = track::table
            .select(track::all_columns)
            .distinct()
            .left_join(agent::table.left_join(room_agent::table))
            .into_boxed();

        if let Some(fq) = req.fq {
            let expr = query_parameters::Expr::from_str(&fq)?;
            match expr {
                query_parameters::Expr::Value(filter) => {
                    // Suppose room_id here
                    and_filter!(query, filter);
                }
                query_parameters::Expr::And(lhs, rhs) => match *lhs {
                    query_parameters::Expr::Value(filter) => {
                        // Suppose room_id here
                        and_filter!(query, filter);

                        match *rhs {
                            query_parameters::Expr::Value(filter) => {
                                // Suppose owner_id here
                                and_filter!(query, filter);
                            }
                            _ => Err(Error::BadRequest)?,
                        }
                    }
                    _ => Err(Error::BadRequest)?,
                },
                _ => Err(Error::BadRequest)?,
            }
        }

        let tracks = query.load::<models::Track>(conn)?;

        Ok(ListResponse::new(&tracks))
    }
}

fn get_agent_room_ids(conn: &PgConnection, agent_id: Uuid) -> Result<Vec<Uuid>> {
    room_agent::table
        .select(room_agent::room_id)
        .filter(room_agent::agent_id.eq(agent_id))
        .get_results(conn)
        .map_err(Error::from)
}
