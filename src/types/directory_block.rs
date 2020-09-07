
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct DirectoryBlockResponse {
    pub(crate) jsonrpc: String,
    pub(crate) id: i64,
    pub(crate) result: DirectoryBlock,
}

#[derive(Serialize, Deserialize)]
pub struct DirectoryBlock {
    pub(crate) header: Header,
    pub(crate) entryblocklist: Vec<EntryBlock>,
}

#[derive(Serialize, Deserialize)]
pub struct EntryBlock {
    pub(crate) chainid: String,
    pub(crate) keymr: String,
}

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub(crate) prevblockkeymr: String,
    pub(crate) sequencenumber: i64,
    pub(crate) timestamp: i64,
}
