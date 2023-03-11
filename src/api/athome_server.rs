use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AtHomeServer {
    pub result: String,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub chapter: Chapter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub hash: String,
    pub data: Vec<String>,
    #[serde(rename = "dataSaver")]
    pub data_saver: Vec<String>,
}
