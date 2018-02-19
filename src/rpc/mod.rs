use jsonrpc_core::{self, to_value, MetaIoHandler, Metadata, Params, Value};

use messages::{EnvelopeSubject, Message};

mod room;

pub type RpcResult = jsonrpc_core::Result<Value>;

#[derive(Clone, Debug, Default)]
pub struct Meta {
    pub subject: EnvelopeSubject,
}

impl Metadata for Meta {}

pub type Server = MetaIoHandler<Meta>;

pub fn build_server() -> Server {
    let mut io = MetaIoHandler::default();

    io.add_method("ping", |params: Params| {
        let _msg: Message = params.parse()?;

        Ok(to_value(Message::Pong).unwrap())
    });

    io.add_method("room.create", room::create);
    io.add_method("room.read", room::read);
    io.add_method("room.update", room::update);
    io.add_method("room.delete", room::delete);
    io.add_method("room.list", room::list);

    io
}
