use jsonrpc_core::Params;

use messages::{agent, EventKind};
use rpc;
use rpc::agent::Rpc as AgentRpc;
use rpc::error::Result;

pub fn call(params: Params, meta: rpc::Meta) {
    if let Ok(mut events) = params.parse::<Vec<EventKind>>() {
        if let Some(event) = events.pop() {
            try_call(meta, event).unwrap_or(());
        }
    }
}

fn try_call(meta: rpc::Meta, event: EventKind) -> Result<()> {
    #[cfg_attr(feature = "cargo-clippy", allow(single_match))]
    match event {
        EventKind::StateUpdate(event) => {
            if !event.is_online() {
                let req = agent::DeleteRequest {
                    id: event.agent_id(),
                };
                let agent_rpc = rpc::agent::RpcImpl {};
                agent_rpc.delete(meta, req)?;
            }
        }
        _ => {}
    }

    Ok(())
}
