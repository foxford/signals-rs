use jsonrpc_core::{MetaIoHandler, Metadata};
use std::sync::mpsc::Sender;

use DbPool;
use messages::{EnvelopeSubject, EventKind};
use rpc::agent::Rpc as AgentRpc;
use rpc::ping::Rpc as PingRpc;
use rpc::room::Rpc as RoomRpc;
use rpc::subscription::Rpc as SubscriptionRpc;

mod ping;
mod agent;
mod room;
mod subscription;

// TODO: remove Default on new jsonrpc_core version
#[derive(Clone, Default)]
pub struct Meta {
    pub subject: EnvelopeSubject,
    pub event_tx: Option<Sender<EventKind>>,
    pub db_pool: Option<DbPool>,
}

impl Metadata for Meta {}

pub type Server = MetaIoHandler<Meta>;

pub fn build_server() -> Server {
    let mut io = MetaIoHandler::default();

    let rpc = ping::RpcImpl {};
    io.extend_with(rpc.to_delegate());

    let rpc = room::RpcImpl {};
    io.extend_with(rpc.to_delegate());

    let rpc = subscription::RpcImpl {};
    io.extend_with(rpc.to_delegate());

    let rpc = agent::RpcImpl {};
    io.extend_with(rpc.to_delegate());

    io
}
