use serde::{Deserialize, Serialize};

/// Metadata associated with file
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Default)]
#[serde(tag = "type")]
pub enum Metadata {
    /// File is just a generic uncategorised file
    #[default]
    File,
    /// File contains textual data and should be displayed as such
    Text,
    /// File is an image with specific dimensions
    Image { width: isize, height: isize },
    /// File is a video with specific dimensions
    Video { width: isize, height: isize },
    /// File is audio
    Audio,
}

/// Representation of a File on Revolt
/// Generated by Autumn
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Default)]
pub struct File {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// Tag / bucket this file was uploaded to
    pub tag: String,
    /// Original filename
    pub filename: String,
    /// Parsed metadata of this file
    pub metadata: Metadata,
    /// Raw content type of this file
    pub content_type: String,
    /// Size of this file (in bytes)
    pub size: isize,

    /// Whether this file was deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Whether this file was reported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported: Option<bool>,

    // ! THE FOLLOWING SHOULD BE DEPRECATED
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,

    /// Id of the object this file is associated with
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}
