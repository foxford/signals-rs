use std::str::FromStr;

use diesel;
use diesel::prelude::*;

use errors::{ErrorKind, Result};
use models;
use rpc;
use schema::{agent, room, track};

use messages::EventKind;
use messages::query_parameters as query;
use messages::track::{CreateEvent, CreateRequest, CreateResponse, DeleteEvent, DeleteRequest,
                      DeleteResponse, ListRequest, ListResponse};

macro_rules! and_filter {
    ($query:ident, $filter:ident) => {
        match $filter {
            query::Filter::RoomId(id) => {
                $query = $query.filter(agent::room_id.eq(id));
            }
            query::Filter::OwnerId(id) => {
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

        let room: models::Room = room::table.find(req.room_id).first(conn)?;

        let changeset = models::NewTrack::from(req);

        let track: models::Track = diesel::insert_into(track::table)
            .values(&changeset)
            .get_result(conn)?;

        let resp = CreateResponse::new(&track);

        let event = CreateEvent::new(room.id, resp.clone());
        let notification_tx = meta.notification_tx.unwrap();
        let event_kind = EventKind::from(event);
        notification_tx.send(event_kind.into()).unwrap();

        Ok(resp)
    }

    fn delete(&self, meta: rpc::Meta, req: DeleteRequest) -> Result<DeleteResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let track: models::Track = track::table
            .select(track::all_columns)
            .inner_join(agent::table)
            .filter(agent::room_id.eq(req.room_id))
            .filter(track::id.eq(req.id))
            .first(conn)?;

        let track = diesel::delete(&track).get_result(conn)?;
        let resp = DeleteResponse::new(&track);

        let event = DeleteEvent::new(req.room_id, resp.clone());
        let notification_tx = meta.notification_tx.unwrap();
        let event_kind = EventKind::from(event);
        notification_tx.send(event_kind.into()).unwrap();

        Ok(resp)
    }

    fn list(&self, meta: rpc::Meta, req: ListRequest) -> Result<ListResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let mut query = track::table
            .select(track::all_columns)
            .left_join(agent::table)
            .into_boxed();

        if let Some(fq) = req.fq {
            let expr = query::Expr::from_str(&fq)?;
            match expr {
                query::Expr::Value(filter) => {
                    // Suppose room_id here
                    and_filter!(query, filter);
                }
                query::Expr::And(lhs, rhs) => match *lhs {
                    query::Expr::Value(filter) => {
                        // Suppose room_id here
                        and_filter!(query, filter);

                        match *rhs {
                            query::Expr::Value(filter) => {
                                // Suppose owner_id here
                                and_filter!(query, filter);
                            }
                            _ => Err(ErrorKind::BadRequest)?,
                        }
                    }
                    _ => Err(ErrorKind::BadRequest)?,
                },
                _ => Err(ErrorKind::BadRequest)?,
            }
        }

        let tracks = query.load::<models::Track>(conn)?;

        Ok(ListResponse::new(&tracks))
    }
}
