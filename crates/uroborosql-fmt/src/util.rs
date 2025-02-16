use crate::config::CONFIG;

/// 設定ファイルに合わせて予約後の大文字・小文字を変換する
pub(crate) fn convert_keyword_case(keyword: &str) -> String {
    CONFIG.read().unwrap().keyword_case.format(keyword)
}

/// 引数の文字列が識別子であれば設定ファイルに合わせて大文字小文字変換をして返す
/// 文字列リテラル、または引用符付き識別子である場合はそのままの文字列を返す
pub(crate) fn convert_identifier_case(identifier: &str) -> String {
    if is_quoted(identifier) {
        identifier.to_owned()
    } else {
        CONFIG.read().unwrap().identifier_case.format(identifier)
    }
}

/// 引数の文字列が引用符付けされているかどうかを判定する。
/// 引用符付けされている場合は true を返す。
pub(crate) fn is_quoted(elem: &str) -> bool {
    (elem.starts_with('"') && elem.ends_with('"'))
        || (elem.starts_with('\'') && elem.ends_with('\''))
        || (elem.starts_with('$') && elem.ends_with('$'))
}

/// 引数の文字列長をタブ数換算した長さを返す
///
/// 例えばtabsize = 4の場合
///
/// to_tab_num(4) => 8
///
/// to_tab_num(4) => 4
pub(crate) fn to_tab_num(len: usize) -> usize {
    if len == 0 {
        0
    } else {
        len / tab_size() + 1
    }
}

/// 設定からタブ長を取得する
pub(crate) fn tab_size() -> usize {
    CONFIG.read().unwrap().tab_size
}

/// 設定の trim_bind_param が true であるとき、引数のバインドパラメータの空白をトリムして返す。
/// 設定が false であるときは、引数をそのまま返す。
pub(crate) fn trim_bind_param(text: String) -> String {
    if CONFIG.read().unwrap().trim_bind_param {
        // 1. /*を削除
        // 2. *\を削除
        // 3. 前後の空白文字を削除
        // 4. /* */付与
        format!(
            "/*{}*/",
            text.trim_start_matches("/*").trim_end_matches("*/").trim()
        )
    } else {
        text
    }
}

/// 引数が定義ファイルで設定した1行の文字数上限を超えていた場合 true を返す
pub(crate) fn is_line_overflow(char_len: usize) -> bool {
    // 1行当たりの上限文字数
    let max_char_per_line = CONFIG.read().unwrap().max_char_per_line;

    if max_char_per_line < 0 {
        // 負の数値が設定されている場合は改行しない
        false
    } else {
        char_len >= max_char_per_line as usize
    }
}
