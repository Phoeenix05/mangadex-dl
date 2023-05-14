use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Feed {
    pub result: String,
    pub response: String,
    pub data: Vec<Datum>,
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Datum {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub datum_type: Option<String>,
    pub attributes: Option<DatumAttributes>,
    pub relationships: Option<Vec<Relationship>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatumAttributes {
    pub title: Option<String>,
    pub volume: Option<String>,
    pub chapter: Option<String>,
    pub pages: Option<i64>,
    #[serde(rename = "translatedLanguage")]
    pub translated_language: Option<String>,
    pub uploader: Option<String>,
    #[serde(rename = "externalUrl")]
    pub external_url: Option<String>,
    #[serde(rename = "version")]
    pub version: Option<i64>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "publishAt")]
    pub publish_at: Option<String>,
    #[serde(rename = "readableAt")]
    pub readable_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationship {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub relationship_type: Option<String>,
    pub related: Option<String>,
    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AtHomeServer {
    pub result: Option<String>,
    #[serde(rename = "baseUrl")]
    pub base_url: Option<String>,
    pub chapter: Option<Chapter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub hash: Option<String>,
    pub data: Option<Vec<String>>,
    #[serde(rename = "dataSaver")]
    pub data_saver: Option<Vec<String>>,
}
