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

use crate::languages::base_struct;

pub fn get_lang() -> base_struct::LangStruct {
    base_struct::LangStruct {
        tool_name_code_indenter: Some("程式碼縮排"),
        tool_describe_code_indenter: Some("功能如其名"),
        tool_name_system_info: Some("系統資訊"),
        tool_describe_system_info: Some("查看系統版本、記憶體等..."),
        tool_name_about: Some("關於"),
        tool_describe_about: Some("關於 positive_toolbox 及第三方專案"),
        main_ui_no_describe: Some("沒有簡介 @_@"),
        ..Default::default()
    }
}
