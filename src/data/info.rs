use serde::{Deserialize, Serialize};

/// License information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "licence", rename_all = "camelCase")]
pub struct License {
    /// Whether the license is valid.
    valid: bool,
    /// User email.
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    /// License expiration date (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    license_expires: Option<String>,
    /// Trial expiration date (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    trial_expires: Option<String>,
}
impl License {
    pub fn default() -> Self {
        License {
            valid: true,
            email: None,
            license_expires: None,
            trial_expires: None
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenSubsonicExtension {
    /// Extension name.
    pub name: String,
    /// Supported version numbers of this extension.
    pub versions: Vec<i32>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(transparent, rename_all = "camelCase")]
pub struct OpenSubsonicExtensionList {
    // Vec of extensions
    open_subsonic_extensions: Vec<OpenSubsonicExtension>,
}

impl OpenSubsonicExtensionList {
    pub fn empty() -> Self {
        OpenSubsonicExtensionList { open_subsonic_extensions: Vec::new() }
    }
}
