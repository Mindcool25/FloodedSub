use serde::{Deserialize, Serialize};
use crate::data::response_enum::{into_response_enum, ResponseEnum};
use crate::{SERVER_NAME, SERVER_VERSION, SUPPORTED_VERSION};
use crate::data::info::{License, OpenSubsonicExtensionList};

// Represents a subsonic status (ok or failed)
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SubsonicStatus {
    Ok,
    Failed,
}

// Response wrapper
#[derive(Serialize)]
pub struct SubsonicResponseWrapper<T>
where
    T: Into<ResponseEnum> + Clone
{
    #[serde(rename = "subsonic-response")]
    pub response: SubsonicResponse<T>,
}

/// The contents of the `"subsonic-response"` object.
#[derive(Deserialize, Serialize)]
pub struct SubsonicResponse<T>
where
    T: Into<ResponseEnum> + Clone
{
    /// `"ok"` or `"failed"`.
    pub status: SubsonicStatus,
    /// Protocol version echoed by the server.
    #[serde(default)]
    #[allow(dead_code)]
    pub version: Option<String>,
    /// Server implementation type (OpenSubsonic extension, e.g. `"navidrome"`).
    #[serde(rename = "type", default)]
    #[allow(dead_code)]
    pub server_type: Option<String>,
    /// Server software version (OpenSubsonic extension).
    #[serde(rename = "serverVersion", default)]
    #[allow(dead_code)]
    pub server_version: Option<String>,
    /// Whether the server supports OpenSubsonic extensions.
    #[serde(rename = "openSubsonic", default)]
    #[allow(dead_code)]
    pub open_subsonic: Option<bool>,
    /// Present only when `status == "failed"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiErrorResponse>,
    /// All remaining fields (the actual endpoint-specific data).
    #[serde(flatten, serialize_with = "into_response_enum")]
    pub data: Option<T>,
}

impl<T> SubsonicResponse<T>
where
    T: Into<ResponseEnum> + Clone
{
    pub fn resp(self) -> SubsonicResponseWrapper<T> {
        SubsonicResponseWrapper { response: self }
    }
}

impl SubsonicResponse<()> {
    pub fn new() -> Self {
        SubsonicResponse {
            status: SubsonicStatus::Ok,
            version: Some(SUPPORTED_VERSION.to_string()),
            server_type: Some(SERVER_NAME.to_string()),
            server_version: Some(SERVER_VERSION.to_string()),
            open_subsonic: Some(true),
            error: None,
            data: None,
        }
    }
}

impl SubsonicResponse<OpenSubsonicExtensionList> {
    pub fn new() -> Self {
        SubsonicResponse {
            status: SubsonicStatus::Ok,
            version: Some(SUPPORTED_VERSION.to_string()),
            server_type: Some(SERVER_NAME.to_string()),
            server_version: Some(SERVER_VERSION.to_string()),
            open_subsonic: Some(true),
            error: None,
            data: Some(OpenSubsonicExtensionList::empty()),
        }
    }
}

impl SubsonicResponse<License> {
    pub fn new() -> Self {
        SubsonicResponse {
            status: SubsonicStatus::Ok,
            version: Some(SUPPORTED_VERSION.to_string()),
            server_type: Some(SERVER_NAME.to_string()),
            server_version: Some(SERVER_VERSION.to_string()),
            open_subsonic: Some(true),
            error: None,
            data: Some(License::default()),
        }
    }
}


#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorResponse {
    pub code: i32,
    pub message: Option<String>,
    pub help_url: Option<String>,
}

