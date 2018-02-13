use models;

// AgentsCreate

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentsCreateRequest {
    pub payload: AgentsCreateRequestPayload,
    cid: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentsCreateRequestPayload {
    pub label: Option<String>,
}

impl AgentsCreateRequest {
    pub fn build_response(self, agent: &models::Agent) -> AgentsCreateResponse {
        AgentsCreateResponse {
            payload: AgentsCreateResponsePayload {
                id: agent.id.to_string(),
                data: AgentsCreateResponseData {
                    label: Some(agent.label.clone()),
                },
            },
            cid: self.cid,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentsCreateResponse {
    payload: AgentsCreateResponsePayload,
    cid: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct AgentsCreateResponsePayload {
    id: String,
    data: AgentsCreateResponseData,
}

type AgentsCreateResponseData = AgentsCreateRequestPayload;

// AgentsCreate

// AgentsRead

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentsReadRequest {
    cid: String,
}

impl AgentsReadRequest {
    pub fn build_response(self, agent: &models::Agent) -> AgentsReadResponse {
        AgentsReadResponse {
            payload: AgentsReadResponsePayload {
                id: agent.id.to_string(),
                data: AgentsReadResponseData {
                    label: Some(agent.label.clone()),
                },
            },
            cid: self.cid,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentsReadResponse {
    payload: AgentsReadResponsePayload,
    cid: String,
}

type AgentsReadResponsePayload = AgentsCreateResponsePayload;
type AgentsReadResponseData = AgentsCreateResponseData;

// AgentsRead

// AgentsUpdate

pub type AgentsUpdateRequest = AgentsCreateRequest;
pub type AgentsUpdateResponse = AgentsCreateResponse;

// AgentsUpdate

// AgentsDelete

pub type AgentsDeleteRequest = AgentsReadRequest;
pub type AgentsDeleteResponse = AgentsReadResponse;

// AgentsDelete

// AgentsList

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentsListRequest {
    cid: String,
}

impl AgentsListRequest {
    pub fn build_response(self, agents: &Vec<models::Agent>) -> AgentsListResponse {
        let payload: Vec<AgentsListResponsePayload> = agents
            .iter()
            .map(|agent| AgentsListResponsePayload {
                id: agent.id.to_string(),
                data: AgentsListResponseData {
                    label: Some(agent.label.clone()),
                },
            })
            .collect();

        AgentsListResponse {
            payload: payload,
            cid: self.cid,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentsListResponse {
    payload: Vec<AgentsListResponsePayload>,
    cid: String,
}

type AgentsListResponsePayload = AgentsReadResponsePayload;
type AgentsListResponseData = AgentsReadResponseData;

// AgentsList
