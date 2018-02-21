use jsonrpc_core::{MetaIoHandler, Metadata};
use std::sync::mpsc::Sender;

use messages::{EnvelopeSubject, EventKind};
use rpc::agent::Rpc as AgentRpc;
use rpc::ping::Rpc as PingRpc;
use rpc::room::Rpc as RoomRpc;

mod ping;
mod agent;
mod room;

// TODO: remove Default on new jsonrpc_core version
#[derive(Clone, Debug, Default)]
pub struct Meta {
    pub subject: EnvelopeSubject,
    pub event_tx: Option<Sender<EventKind>>,
}

impl Metadata for Meta {}

pub type Server = MetaIoHandler<Meta>;

pub fn build_server() -> Server {
    let mut io = MetaIoHandler::default();

    let rpc = ping::RpcImpl {};
    io.extend_with(rpc.to_delegate());

    let rpc = room::RpcImpl {};
    io.extend_with(rpc.to_delegate());

    let rpc = agent::RpcImpl {};
    io.extend_with(rpc.to_delegate());

    io
}
