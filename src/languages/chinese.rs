use crate::languages::base_struct;

pub fn get_lang() -> base_struct::LangStruct {
    base_struct::LangStruct {
        tool_name_code_indenter: "程式碼縮排",
        tool_name_system_info: "系統資訊",
        tool_name_about: "關於",
        main_ui_no_describe: "沒有簡介 @_@",
    }
}
