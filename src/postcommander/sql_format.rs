use std::collections::HashSet;

const SQL_KEYWORDS: &[&str] = &[
    "select", "from", "where", "and", "or", "not", "in", "like", "ilike",
    "between", "is", "null", "true", "false", "order", "by", "asc", "desc",
    "limit", "offset", "group", "having", "join", "inner", "left", "right",
    "outer", "cross", "on", "as", "distinct", "all", "union", "intersect",
    "except", "exists", "case", "when", "then", "else", "end", "cast",
    "insert", "into", "values", "update", "set", "delete", "create", "alter",
    "drop", "table", "index", "view", "schema", "database", "truncate",
    "constraint", "primary", "key", "foreign", "references", "unique",
    "default", "check", "cascade", "nulls", "first", "last", "returning",
    "with", "recursive", "over", "partition", "rows", "range", "unbounded",
    "preceding", "following", "current", "row", "coalesce", "nullif",
];

fn get_keywords_set() -> HashSet<&'static str> {
    SQL_KEYWORDS.iter().copied().collect()
}

pub fn format_sql(sql: &str) -> String {
    let keywords = get_keywords_set();
    let mut result = String::with_capacity(sql.len());
    let mut word_start = 0;
    let mut in_string = false;
    let mut string_char = ' ';
    let chars: Vec<char> = sql.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if !in_string && (c == '\'' || c == '"') {
            in_string = true;
            string_char = c;
        } else if in_string && c == string_char {
            in_string = false;
        }

        let is_word_char = c.is_alphanumeric() || c == '_';

        if !is_word_char || i == chars.len() - 1 {
            let end = if is_word_char { i + 1 } else { i };
            if end > word_start {
                let word: String = chars[word_start..end].iter().collect();
                let lower = word.to_lowercase();

                if !in_string && keywords.contains(lower.as_str()) {
                    result.push_str(&word.to_uppercase());
                } else {
                    result.push_str(&word);
                }
            }

            if !is_word_char {
                result.push(c);
            }
            word_start = i + 1;
        }
    }

    result
}

pub fn capitalize_sql_keyword(word: &str) -> Option<String> {
    let lower = word.to_lowercase();
    if SQL_KEYWORDS.contains(&lower.as_str()) {
        Some(word.to_uppercase())
    } else {
        None
    }
}

pub fn maybe_capitalize_last_word(text: &str, trigger_char: char) -> Option<(usize, usize, String)> {
    if !matches!(trigger_char, ' ' | '\n' | '\t' | ',' | '(' | ')' | ';') {
        return None;
    }

    let text_before = text.trim_end_matches(trigger_char);
    if text_before.is_empty() {
        return None;
    }

    let word_start = text_before
        .rfind(|c: char| c.is_whitespace() || c == ',' || c == '(' || c == ')' || c == ';')
        .map(|i| i + 1)
        .unwrap_or(0);

    let word = &text_before[word_start..];
    if word.is_empty() {
        return None;
    }

    if word == word.to_uppercase() {
        return None;
    }

    capitalize_sql_keyword(word).map(|upper| {
        (word_start, word_start + word.len(), upper)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_select() {
        assert_eq!(
            capitalize_sql_keyword("select"),
            Some("SELECT".to_string())
        );
    }

    #[test]
    fn test_capitalize_already_upper() {
        assert_eq!(
            capitalize_sql_keyword("SELECT"),
            Some("SELECT".to_string())
        );
    }

    #[test]
    fn test_no_capitalize_non_keyword() {
        assert_eq!(capitalize_sql_keyword("users"), None);
    }

    #[test]
    fn test_maybe_capitalize_simple() {
        let result = maybe_capitalize_last_word("select ", ' ');
        assert_eq!(result, Some((0, 6, "SELECT".to_string())));
    }

    #[test]
    fn test_maybe_capitalize_in_query() {
        let result = maybe_capitalize_last_word("SELECT * from ", ' ');
        assert_eq!(result, Some((9, 13, "FROM".to_string())));
    }

    #[test]
    fn test_maybe_capitalize_already_upper() {
        let result = maybe_capitalize_last_word("SELECT ", ' ');
        assert_eq!(result, None);
    }

    #[test]
    fn test_maybe_capitalize_non_keyword() {
        let result = maybe_capitalize_last_word("SELECT * FROM users ", ' ');
        assert_eq!(result, None);
    }
}
