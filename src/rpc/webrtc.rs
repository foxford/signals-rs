use messages::webrtc::{AnswerRequest, CandidateRequest, OfferRequest};
use messages::Method;
use rpc;
use rpc::error::Result;

build_rpc_trait! {
    pub trait Rpc {
        type Metadata;

        #[rpc(meta, name = "webrtc.offer")]
        fn offer(&self, Self::Metadata, OfferRequest) -> Result<Vec<()>>;

        #[rpc(meta, name = "webrtc.answer")]
        fn answer(&self, Self::Metadata, AnswerRequest) -> Result<Vec<()>>;

        #[rpc(meta, name = "webrtc.candidate")]
        fn candidate(&self, Self::Metadata, CandidateRequest) -> Result<Vec<()>>;
    }
}

pub struct RpcImpl;

impl Rpc for RpcImpl {
    type Metadata = rpc::Meta;

    fn offer(&self, meta: rpc::Meta, req: OfferRequest) -> Result<Vec<()>> {
        let method = Method::from(req);
        let notification_tx = meta.notification_tx.unwrap();
        notification_tx.send(method.into()).unwrap();

        Ok(vec![])
    }

    fn answer(&self, meta: rpc::Meta, req: AnswerRequest) -> Result<Vec<()>> {
        let method = Method::from(req);
        let notification_tx = meta.notification_tx.unwrap();
        notification_tx.send(method.into()).unwrap();

        Ok(vec![])
    }

    fn candidate(&self, meta: rpc::Meta, req: CandidateRequest) -> Result<Vec<()>> {
        let method = Method::from(req);
        let notification_tx = meta.notification_tx.unwrap();
        notification_tx.send(method.into()).unwrap();

        Ok(vec![])
    }
}
