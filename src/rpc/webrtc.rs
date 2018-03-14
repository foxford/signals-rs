use errors::Result;
use messages::webrtc::{OfferRequest};
use rpc;

build_rpc_trait! {
    pub trait Rpc {
        type Metadata;

        #[rpc(meta, name = "webrtc.offer")]
        fn offer(&self, Self::Metadata, OfferRequest) -> Result<()>;

        // #[rpc(meta, name = "webrtc.answer")]
        // fn answer(&self, Self::Metadata, ReadRequest) -> Result<()>;

        // #[rpc(meta, name = "webrtc.candidate")]
        // fn candidate(&self, Self::Metadata, DeleteRequest) -> Result<()>;
    }
}

pub struct RpcImpl;

impl Rpc for RpcImpl {
    type Metadata = rpc::Meta;

    fn offer(&self, meta: rpc::Meta, req: OfferRequest) -> Result<()> {
        let conn = establish_connection!(meta.db_pool.unwrap());

        let room: models::Room = diesel::insert_into(room::table)
            .default_values()
            .get_result(conn)?;

        Ok(())
    }
}
