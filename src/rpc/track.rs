use std::str::FromStr;

use diesel;
use diesel::prelude::*;

use errors::{ErrorKind, Result};
use models;
use rpc;
use schema::{agent, local_track, remote_track, room};

use messages::EventKind;
use messages::query_parameters as query;
use messages::track::{CreateEvent, CreateRequest, CreateResponse, DeleteEvent, DeleteRequest,
                      DeleteResponse, ListRequest, ListResponse, RegisterRequest,
                      RegisterResponse, UnregisterRequest, UnregisterResponse, UpdateEvent};

macro_rules! and_filter {
    ($query:ident, $filter:ident) => {
        match $filter {
            query::Filter::RoomId(id) => {
                $query = $query.filter(agent::room_id.eq(id));
            }
            query::Filter::OwnerId(id) => {
                $query = $query.filter(local_track::owner_id.eq(id));
            }
            query::Filter::HolderId(id) => {
                $query = $query.filter(remote_track::agent_id.eq(id));
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

        #[rpc(meta, name = "track.register")]
        fn register(&self, Self::Metadata, RegisterRequest) -> Result<RegisterResponse>;

        #[rpc(meta, name = "track.unregister")]
        fn unregister(&self, Self::Metadata, UnregisterRequest) -> Result<UnregisterResponse>;

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

        let data = req.data;
        let changeset = models::NewLocalTrack {
            stream_id: data.stream_id,
            track_id: data.track_id,
            device: data.device,
            kind: data.kind,
            label: data.label,
            owner_id: data.owner_id,
        };

        let track: models::LocalTrack = diesel::insert_into(local_track::table)
            .values(&changeset)
            .get_result(conn)?;

        let resp = CreateResponse::new(&track, &[]);

        let event = CreateEvent::new(room.id, resp.clone());
        let notification_tx = meta.notification_tx.unwrap();
        let event_kind = EventKind::from(event);
        notification_tx.send(event_kind.into()).unwrap();

        Ok(resp)
    }

    fn delete(&self, meta: rpc::Meta, req: DeleteRequest) -> Result<DeleteResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let track: models::LocalTrack = local_track::table
            .select(local_track::all_columns)
            .inner_join(agent::table)
            .filter(agent::room_id.eq(req.room_id))
            .filter(local_track::id.eq(req.id))
            .first(conn)?;

        let track = diesel::delete(&track).get_result(conn)?;
        let resp = DeleteResponse::new(&track, &[]);

        let event = DeleteEvent::new(req.room_id, resp.clone());
        let notification_tx = meta.notification_tx.unwrap();
        let event_kind = EventKind::from(event);
        notification_tx.send(event_kind.into()).unwrap();

        Ok(resp)
    }

    fn register(&self, meta: rpc::Meta, req: RegisterRequest) -> Result<RegisterResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let track = local_track::table
            .filter(local_track::stream_id.eq(req.data.stream_id))
            .filter(local_track::track_id.eq(req.data.track_id))
            .get_result::<models::LocalTrack>(conn)?;

        let changeset = models::NewRemoteTrack {
            local_track_id: track.id,
            agent_id: req.data.agent_id,
        };
        diesel::insert_into(remote_track::table)
            .values(&changeset)
            .execute(conn)?;

        let remote_tracks = models::RemoteTrack::belonging_to(&track).load(conn)?;

        let resp = RegisterResponse::new(&track, &remote_tracks);

        let event = UpdateEvent::new(req.room_id, resp.clone());
        let notification_tx = meta.notification_tx.unwrap();
        let event_kind = EventKind::from(event);
        notification_tx.send(event_kind.into()).unwrap();

        Ok(resp)
    }

    fn unregister(&self, meta: rpc::Meta, req: UnregisterRequest) -> Result<UnregisterResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let track = local_track::table
            .filter(local_track::stream_id.eq(req.data.stream_id))
            .filter(local_track::track_id.eq(req.data.track_id))
            .get_result::<models::LocalTrack>(conn)?;

        diesel::delete(
            remote_track::table
                .filter(remote_track::local_track_id.eq(track.id))
                .filter(remote_track::agent_id.eq(req.data.agent_id)),
        ).execute(conn)?;

        let remote_tracks = models::RemoteTrack::belonging_to(&track).load(conn)?;

        let resp = UnregisterResponse::new(&track, &remote_tracks);

        let event = UpdateEvent::new(req.room_id, resp.clone());
        let notification_tx = meta.notification_tx.unwrap();
        let event_kind = EventKind::from(event);
        notification_tx.send(event_kind.into()).unwrap();

        Ok(resp)
    }

    fn list(&self, meta: rpc::Meta, req: ListRequest) -> Result<ListResponse> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let mut query = local_track::table
            .select(local_track::all_columns)
            .left_join(agent::table)
            .left_join(remote_track::table)
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
                                // Suppose owner_id or holders.id here
                                and_filter!(query, filter);
                            }
                            query::Expr::Parenthesis(ex) => {
                                // To prevent "use of collaterally moved value: `(ex:messages::query_parameters::Expr::Or).1`" error
                                let ex = *ex;

                                match ex {
                                    query::Expr::Or(lhs, rhs) => match (*lhs, *rhs) {
                                        (query::Expr::Value(lhs), query::Expr::Value(rhs)) => {
                                            match (lhs, rhs) {
                                                (
                                                    query::Filter::OwnerId(o_id),
                                                    query::Filter::HolderId(h_id),
                                                ) => {
                                                    query = query
                                                        .filter(local_track::owner_id.eq(o_id))
                                                        .or_filter(remote_track::agent_id.eq(h_id));
                                                }
                                                _ => Err(ErrorKind::BadRequest)?,
                                            }
                                        }
                                        _ => Err(ErrorKind::BadRequest)?,
                                    },
                                    _ => Err(ErrorKind::BadRequest)?,
                                }
                            }
                            _ => Err(ErrorKind::BadRequest)?,
                        }
                    }
                    _ => Err(ErrorKind::BadRequest)?,
                },
                _ => Err(ErrorKind::BadRequest)?,
            }
        }

        let local_tracks = query.load::<models::LocalTrack>(conn)?;
        let remote_tracks = models::RemoteTrack::belonging_to(&local_tracks)
            .load::<models::RemoteTrack>(conn)?
            .grouped_by(&local_tracks);

        let data = local_tracks
            .into_iter()
            .zip(remote_tracks)
            .collect::<Vec<_>>();

        Ok(ListResponse::new(&data))
    }
}
