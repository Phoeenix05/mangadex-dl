use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Feed {
    result: String,
    response: String,
    data: Vec<Datum>,
    limit: i64,
    offset: i64,
    total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Datum {
    id: Option<String>,
    #[serde(rename = "type")]
    datum_type: Option<String>,
    attributes: Option<DatumAttributes>,
    relationships: Option<Vec<Relationship>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatumAttributes {
    title: Option<String>,
    volume: Option<String>,
    chapter: Option<String>,
    pages: Option<i64>,
    #[serde(rename = "translatedLanguage")]
    translated_language: Option<String>,
    uploader: Option<String>,
    #[serde(rename = "externalUrl")]
    external_url: Option<String>,
    #[serde(rename = "version")]
    version: Option<i64>,
    #[serde(rename = "createdAt")]
    created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    #[serde(rename = "publishAt")]
    publish_at: Option<String>,
    #[serde(rename = "readableAt")]
    readable_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationship {
    id: Option<String>,
    #[serde(rename = "type")]
    relationship_type: Option<String>,
    related: Option<String>,
    attributes: Option<serde_json::Value>,
}

// #[derive(Serialize, Deserialize)]
// pub struct RelationshipAttributes {}
