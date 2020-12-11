use super::*;

pub async fn chain_head(client: &Factom, chainid: &str) -> Result<ApiResponse<ChainHead>> {
    let mut req = ApiRequest::new("chain-head");
    req.params.insert("chainid".to_string(), json!(chainid));
    let response = requests::factomd_call(client, req).await;
    parse(response).await
}

pub async fn commit_chain(api: &Factom, message: &str) -> Result<ApiResponse<CommitChain>> {
    let mut req = ApiRequest::new("commit-chain");
    req.params.insert("message".to_string(), json!(message));
    let response = factomd_call(api, req).await;
    parse(response).await
}

pub async fn reveal_chain(api: &Factom, entry: &str) -> Result<ApiResponse<RevealChain>> {
    let mut req = ApiRequest::new("reveal-chain");
    req.params.insert("entry".to_string(), json!(entry));
    let response = factomd_call(api, req).await;
    parse(response).await
}

/// chain-head function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChainHead {
    pub chainhead: String,
    pub chaininprocesslist: bool,
}

/// commit-chain function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitChain {
    pub message: String,
    pub txid: String,
    pub entryhash: String,
    #[serde(alias = "chainidhash")]
    pub chainid: String,
}

/// reveal-chain function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevealChain {
    pub message: String,
    pub entryhash: String,
    #[serde(alias = "chainidhash")]
    pub chainid: String,
}
