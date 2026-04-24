use serde;

use crate::languages::base::SupportedLanguages;

pub const SETTING_FILE_NAME: &str = "ptb_settings.json";

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct PTBSettings {
    normal: NormalSettings,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct NormalSettings {
    language: SupportedLanguages,
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
