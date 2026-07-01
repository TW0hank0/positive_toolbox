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

use crate::languages::{self, base::PTBLanguages};

/* pub const LANG: PTBLanguages = PTBLanguages {
    tool_info: languages::base::LangTypeToolInfo {
        code_indenter_name: Some(String::from("程式碼縮排")),
        code_indenter_describe: Some(String::from("功能如其名")),
        system_info_name: Some(String::from("系統資訊")),
        system_info_describe: Some(String::from("查看系統版本、記憶體等...")),
        about_name: Some(String::from("關於")),
        about_describe: Some(String::from("關於 positive_toolbox 專案")),
    },
    home_page: languages::base::LangTypeHomePage {
        tool_no_describe: Some(String::from("沒有簡介 @_@")),
    },
}; */

pub fn get_lang() -> PTBLanguages {
    PTBLanguages {
        tool_info: languages::base::LangTypeToolInfo {
            code_indenter_name: Some(String::from("程式碼縮排")),
            code_indenter_describe: Some(String::from("功能如其名")),
            system_info_name: Some(String::from("系統資訊")),
            system_info_describe: Some(String::from("查看系統版本、記憶體等...")),
            about_name: Some(String::from("關於")),
            about_describe: Some(String::from("關於 positive_toolbox 專案")),
        },
        home_page: languages::base::LangTypeHomePage {
            tool_no_describe: Some(String::from("沒有簡介 @_@")),
        },
    }
}
