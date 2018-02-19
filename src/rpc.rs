use jsonrpc_core::{MetaIoHandler, Metadata, Params};
use serde_json;

use messages::{EnvelopeSubject, Message};

#[derive(Clone, Debug, Default)]
pub struct Meta {
    pub subject: EnvelopeSubject,
}

impl Metadata for Meta {}

pub type Server = MetaIoHandler<Meta>;

pub fn build_server() -> Server {
    let mut io = MetaIoHandler::default();

    io.add_method("ping", |params: Params| {
        let _msg: Message = params.parse().unwrap();

        Ok(serde_json::to_value(Message::Pong).unwrap())
    });

    io
}
