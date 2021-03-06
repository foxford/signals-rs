use jsonrpc_core::{MetaIoHandler, Metadata};

use std::sync::mpsc::Sender;

use messages::{EnvelopeSubject, Notification};
use rpc::agent::Rpc as AgentRpc;
use rpc::ping::Rpc as PingRpc;
use rpc::room::Rpc as RoomRpc;
use rpc::subscription::Rpc as SubscriptionRpc;
use rpc::track::Rpc as TrackRpc;
use rpc::webrtc::Rpc as WebrtcRpc;
use DbPool;

mod agent;
mod error;
mod event;
mod ping;
mod room;
mod subscription;
mod track;
mod webrtc;

// TODO: remove Default on new jsonrpc_core version
#[derive(Clone, Default)]
pub struct Meta {
    pub subject: EnvelopeSubject,
    pub notification_tx: Option<Sender<Notification>>,
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

    let rpc = track::RpcImpl {};
    io.extend_with(rpc.to_delegate());

    let rpc = webrtc::RpcImpl {};
    io.extend_with(rpc.to_delegate());

    io.add_notification_with_meta("event", event::call);

    io
}
