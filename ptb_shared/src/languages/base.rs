// SPDX-License-Identifier: AGPL-3.0-only
// 著作權所有 (C) 2026 TW0hank0
//
// 本檔案屬於 positive_toolbox 專案的一部分。
// 專案儲存庫：https://github.com/TW0hank0/positive_toolbox
//
// 本程式為自由軟體：您可以根據自由軟體基金會發佈的 GNU Affero 通用公共授權條款
// 第 3 版（僅此版本）重新發佈及/或修改本程式。
//
// 本程式的發佈是希望它能發揮功用，但不提供任何擔保；
// 甚至沒有隱含的適銷性或特定目的適用性擔保。詳見 GNU Affero 通用公共授權條款。
//
// 您應該已經收到一份 GNU Affero 通用公共授權條款副本。
// 如果沒有，請參見 <https://www.gnu.org/licenses/>。

use std::{self, fmt::Display};

use serde;

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum SupportedLanguages {
    Chinese,
    English,
}

impl Display for SupportedLanguages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::English => "English",
                Self::Chinese => "中文",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct LangStruct {
    pub tool_name_code_indenter: Option<&'static str>,
    pub tool_describe_code_indenter: Option<&'static str>,
    pub tool_name_system_info: Option<&'static str>,
    pub tool_describe_system_info: Option<&'static str>,
    pub tool_name_about: Option<&'static str>,
    pub tool_describe_about: Option<&'static str>,
    pub main_ui_no_describe: Option<&'static str>,
}

impl std::default::Default for LangStruct {
    fn default() -> Self {
        Self {
            tool_name_code_indenter: None,
            tool_describe_code_indenter: None,
            tool_name_system_info: None,
            tool_describe_system_info: None,
            tool_name_about: None,
            tool_describe_about: None,
            main_ui_no_describe: None,
        }
    }
}

#[macro_export]
macro_rules! lang_get {
    ($lang:expr, $field:ident) => {{
        use ptb_shared::languages;
        $lang
            .$field
            .as_ref()
            .unwrap_or(languages::chinese::LANG.$field.as_ref().unwrap())
    }};
}
