use errors::Result;
use messages::subscription::{CreateRequest, CreateResponse};
use topic::AppTopic;

build_rpc_trait! {
    pub trait Rpc {
        #[rpc(name = "subscription.create")]
        fn create(&self, CreateRequest) -> Result<CreateResponse>;
    }
}

pub struct RpcImpl;

impl Rpc for RpcImpl {
    fn create(&self, req: CreateRequest) -> Result<CreateResponse> {
        let topic = AppTopic {
            room_id: req.room_id,
            resource: req.data.resource,
        };

        let resp = CreateResponse::new(topic);
        Ok(resp)
    }
}
