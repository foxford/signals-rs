use diesel;
use diesel::prelude::*;
use jsonrpc_core::{to_value, Params};
use messages::room;
use models;
use rpc::RpcResult;
use schema::rooms;

pub fn create(params: Params) -> RpcResult {
    let req: room::CreateRequest = params.parse()?;

    // FIXME: use connection pool
    let conn = establish_connection();

    // FIXME: don't unwrap
    let room: models::Room = diesel::insert_into(rooms::table)
        .values(&req.data)
        .get_result(&conn)
        .unwrap();

    let resp = room::CreateResponse::new(&room);
    // FIXME: don't unwrap
    Ok(to_value(resp).unwrap())
}

pub fn read(params: Params) -> RpcResult {
    let req: room::ReadRequest = params.parse()?;

    // FIXME: use connection pool
    let conn = establish_connection();

    // FIXME: don't unwrap
    let room: models::Room = rooms::table.find(req.room_id).first(&conn).unwrap();
    let resp = room::ReadResponse::new(&room);

    // FIXME: don't unwrap
    Ok(to_value(resp).unwrap())
}

pub fn update(params: Params) -> RpcResult {
    let req: room::UpdateRequest = params.parse()?;

    // FIXME: use connection pool
    let conn = establish_connection();

    // FIXME: don't unwrap
    let room = rooms::table.find(req.room_id);
    let room: models::Room = diesel::update(room)
        .set(&req.data)
        .get_result(&conn)
        .unwrap();
    let resp = room::UpdateResponse::new(&room);

    // FIXME: don't unwrap
    Ok(to_value(resp).unwrap())
}

pub fn delete(params: Params) -> RpcResult {
    let req: room::DeleteRequest = params.parse()?;

    // FIXME: use connection pool
    let conn = establish_connection();

    // FIXME: don't unwrap
    let room = rooms::table.find(req.room_id);
    let room: models::Room = diesel::delete(room).get_result(&conn).unwrap();
    let resp = room::DeleteResponse::new(&room);

    // FIXME: don't unwrap
    Ok(to_value(resp).unwrap())
}

pub fn list(_params: Params) -> RpcResult {
    // FIXME: use connection pool
    let conn = establish_connection();

    // FIXME: don't unwrap
    let rooms = rooms::table.load::<models::Room>(&conn).unwrap();
    let resp = room::ListResponse::new(&rooms);

    // FIXME: don't unwrap
    Ok(to_value(resp).unwrap())
}

fn establish_connection() -> PgConnection {
    let database_url = ::std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap()
}
