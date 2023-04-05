use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapRoot {
    pub id: String,
    pub download: String,
    pub version: i64,
    pub name: String,
    pub song: String,
    pub author: Vec<String>,
    pub difficulty: i64,
    #[serde(rename = "difficulty_name")]
    pub difficulty_name: String,
    pub stars: i64,
    #[serde(rename = "length_ms")]
    pub length_ms: i64,
    #[serde(rename = "note_count")]
    pub note_count: i64,
    #[serde(rename = "has_cover")]
    pub has_cover: bool,
    pub broken: bool,
    pub tags: Vec<String>,
    #[serde(rename = "content_warnings")]
    pub content_warnings: Vec<Value>,
    #[serde(rename = "note_data_offset")]
    pub note_data_offset: i64,
    #[serde(rename = "note_data_length")]
    pub note_data_length: i64,
    #[serde(rename = "music_format")]
    pub music_format: Option<String>,
    #[serde(rename = "music_offset")]
    pub music_offset: Option<i64>,
    #[serde(rename = "music_length")]
    pub music_length: Option<i64>,
}
pub type DbRoot = serde_json::value::Map<String,Value>;