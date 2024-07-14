use lingua::{Language, LanguageDetectorBuilder};

/// 获取翻译目标语言
///
/// Get the translation target language
pub fn lang(text: &mut String) -> String {
    let first_lang = "zh";
    let second_lang = "en";
    let languages = vec![Language::Chinese, Language::English];
    let detector = LanguageDetectorBuilder::from_languages(&languages).build();
    // 第一语言翻译为第二语言，其他翻译为第一语言
    // Translate the first language into the second language, and the other language into the first
    let kind = match detector.detect_language_of(text.as_str()) {
        Some(lang) => {
            handle_newlines(text, lang);
            match lang {
                Language::Chinese => second_lang,
                Language::English => first_lang,
            }
        }
        None => first_lang,
    };
    kind.to_string()
}

/// 将单个的换行符（\r\n 或者 \n）换成空格（en）或去除（zh）
///
/// Convert single newlines (\r\n or \n) to spaces (for en) or delete them (for zh)
fn handle_newlines(content: &mut String, lang: Language) {
    let to_replace = match lang {
        Language::Chinese => "",
        Language::English => " ",
    };

    // 首先处理 \r\n
    // First deal with \r\n
    let newlines: Vec<usize> = content.match_indices("\r\n").map(|m| m.0).collect();
    let mut single_newlines = Vec::new();
    for (n, newline) in newlines.iter().enumerate() {
        // 如果某个换行符前后还有换行符, 则不是单个的换行符
        // If a newline has another newline before or after it, it is not a single newline
        if n > 0 && newlines[n - 1] == *newline - 2 {
            continue;
        }
        if n < newlines.len() - 1 && newlines[n + 1] == *newline + 2 {
            continue;
        }
        single_newlines.push(*newline);
    }
    // 从后往前替换，以免替换过程中改变未替换的换行符的位置
    // Replace in reverse order in order not to change the positions of other newlines during replacements
    for i in single_newlines.iter().rev() {
        // 如果换行前面是“-”则不需要空格
        // If there is a "-" before newline then space is not needed
        if *i > 0 && content.as_bytes()[*i - 1] == b'-' {
            content.replace_range(*i..*i + 2, "");
        } else {
            content.replace_range(*i..*i + 2, to_replace);
        }
    }

    // 处理 \n （仅当没有找到 \r\n 时）
    // Deal with \n (only when \r\n is not found)
    if newlines.is_empty() {
        let newlines: Vec<usize> = content.match_indices("\n").map(|m| m.0).collect();
        let mut single_newlines = Vec::new();
        for (n, newline) in newlines.iter().enumerate() {
            // 如果某个换行符前后还有换行符, 则不是单个的换行符
            // If a newline has another newline before or after it, it is not a single newline
            if n > 0 && newlines[n - 1] == *newline - 1 {
                continue;
            }
            if n < newlines.len() - 1 && newlines[n + 1] == *newline + 1 {
                continue;
            }
            single_newlines.push(*newline);
        }
        // 从后往前替换，以免替换过程中改变未替换的换行符的位置
        // Replace in reverse order in order not to change the positions of other newlines during replacements
        for i in single_newlines.iter().rev() {
            // 如果换行前面是“-”则不需要空格
            // If there is a "-" before newline then space is not needed
            if *i > 0 && content.as_bytes()[*i - 1] == b'-' {
                content.replace_range(*i..*i + 1, "");
            } else {
                content.replace_range(*i..*i + 1, to_replace);
            }
        }
    }
}
