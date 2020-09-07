extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct Result {
    pub(crate) jsonrpc: String,
    pub(crate) id: i64,
    pub(crate) result: CurrentiMinuteResponse,
}

#[derive(Serialize, Deserialize)]
pub struct CurrentMinuteResponse {
    pub(crate) leaderheight: i64,
    pub(crate) directoryblockheight: i64,
    pub(crate) minute: i64,
    pub(crate) currentblockstarttime: f64,
    pub(crate) currentminutestarttime: f64,
    pub(crate) currenttime: f64,
    pub(crate) directoryblockinseconds: i64,
    pub(crate) stalldetected: bool,
    pub(crate) faulttimeout: i64,
    pub(crate) roundtimeout: i64,
}
