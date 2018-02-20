use jsonrpc_core::{self, to_value, MetaIoHandler, Metadata, Params, Value};

use messages::{EnvelopeSubject, Message};
use rpc::ping::Rpc as PingRpc;
use rpc::room::Rpc as RoomRpc;

mod ping;
mod room;

#[derive(Clone, Debug, Default)]
pub struct Meta {
    pub subject: EnvelopeSubject,
}

impl Metadata for Meta {}

pub type Server = MetaIoHandler<Meta>;

pub fn build_server() -> Server {
    let mut io = MetaIoHandler::default();

    let rpc = ping::RpcImpl {};
    io.extend_with(rpc.to_delegate());

    let rpc = room::RpcImpl {};
    io.extend_with(rpc.to_delegate());

    io
}
