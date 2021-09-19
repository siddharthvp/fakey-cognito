use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct SoftwareTokenMfaSettingsType {
    pub enabled: Option<bool>,
    pub prefrred_mfa: Option<bool>,
}