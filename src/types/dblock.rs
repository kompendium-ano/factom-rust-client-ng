extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct DBlockResponse {
    pub(crate) jsonrpc: String,
    pub(crate) id: i64,
    pub(crate) result: DBlockResult,
}

#[derive(Serialize, Deserialize)]
pub struct DBlockResult {
    pub(crate) dblock: Dblock,
    pub(crate) rawdata: String,
}

#[derive(Serialize, Deserialize)]
pub struct Dblock {
    pub(crate) header: Header,
    pub(crate) dbentries: Vec<DBEntry>,
    pub(crate) dbhash: String,
    pub(crate) keymr: String,
}

#[derive(Serialize, Deserialize)]
pub struct DBEntry {
    pub(crate) chainid: String,
    pub(crate) keymr: String,
}

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub(crate) version: i64,
    pub(crate) networkid: i64,
    pub(crate) bodymr: String,
    pub(crate) prevkeymr: String,
    pub(crate) prevfullhash: String,
    pub(crate) timestamp: i64,
    pub(crate) dbheight: i64,
    pub(crate) blockcount: i64,
    pub(crate) chainid: String,
}