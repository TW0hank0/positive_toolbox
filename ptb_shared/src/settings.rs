use serde;

use crate::languages::base::SupportedLanguages;

pub const SETTING_FILE_NAME: &str = "ptb_settings.json";

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct PTBSettings {
    pub normal: NormalSettings,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct NormalSettings {
    pub language: SupportedLanguages,
}

impl Default for PTBSettings {
    fn default() -> Self {
        Self {
            normal: NormalSettings {
                language: SupportedLanguages::Chinese,
            },
        }
    }
}
