use std::collections::HashSet;
use std::sync::LazyLock;

pub const KEYWORD: &str = "\x1b[1;34m";
pub const STRING: &str = "\x1b[32m";
pub const NUMBER: &str = "\x1b[33m";
pub const COMMENT: &str = "\x1b[2m";
pub const DIM: &str = "\x1b[2m";
pub const BOLD: &str = "\x1b[1m";
pub const RED: &str = "\x1b[31m";
pub const RESET: &str = "\x1b[0m";

pub static KEYWORDS: &[&str] = &[
    "ABORT",
    "ACTION",
    "ADD",
    "AFTER",
    "ALL",
    "ALTER",
    "ANALYZE",
    "AND",
    "AS",
    "ASC",
    "ATTACH",
    "AUTOINCREMENT",
    "BEFORE",
    "BEGIN",
    "BETWEEN",
    "BY",
    "CASCADE",
    "CASE",
    "CAST",
    "CHECK",
    "COLLATE",
    "COLUMN",
    "COMMIT",
    "CONFLICT",
    "CONSTRAINT",
    "CREATE",
    "CROSS",
    "CURRENT_DATE",
    "CURRENT_TIME",
    "CURRENT_TIMESTAMP",
    "DATABASE",
    "DEFAULT",
    "DEFERRABLE",
    "DEFERRED",
    "DELETE",
    "DESC",
    "DETACH",
    "DISTINCT",
    "DROP",
    "EACH",
    "ELSE",
    "END",
    "ESCAPE",
    "EXCEPT",
    "EXCLUSIVE",
    "EXISTS",
    "EXPLAIN",
    "FAIL",
    "FOR",
    "FOREIGN",
    "FROM",
    "FULL",
    "GLOB",
    "GROUP",
    "HAVING",
    "IF",
    "IGNORE",
    "IMMEDIATE",
    "IN",
    "INDEX",
    "INDEXED",
    "INITIALLY",
    "INNER",
    "INSERT",
    "INSTEAD",
    "INTERSECT",
    "INTO",
    "IS",
    "ISNULL",
    "JOIN",
    "KEY",
    "LEFT",
    "LIKE",
    "LIMIT",
    "MATCH",
    "NATURAL",
    "NO",
    "NOT",
    "NOTNULL",
    "NULL",
    "OF",
    "OFFSET",
    "ON",
    "OR",
    "ORDER",
    "OUTER",
    "PLAN",
    "PRAGMA",
    "PRIMARY",
    "QUERY",
    "RAISE",
    "REFERENCES",
    "REGEXP",
    "REINDEX",
    "RELEASE",
    "RENAME",
    "REPLACE",
    "RESTRICT",
    "RETURNING",
    "RIGHT",
    "ROLLBACK",
    "ROW",
    "SAVEPOINT",
    "SELECT",
    "SET",
    "TABLE",
    "TEMP",
    "TEMPORARY",
    "THEN",
    "TO",
    "TRANSACTION",
    "TRIGGER",
    "UNION",
    "UNIQUE",
    "UPDATE",
    "USING",
    "VACUUM",
    "VALUES",
    "VIEW",
    "VIRTUAL",
    "WHEN",
    "WHERE",
    "WITH",
    "WITHOUT",
];

static LOOKUP: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| KEYWORDS.iter().copied().collect());

pub fn paint(line: &str) -> String {
    let characters: Vec<char> = line.chars().collect();
    let mut out = String::with_capacity(line.len() * 2);
    let mut index = 0;

    while index < characters.len() {
        let character = characters[index];

        if character == '-' && characters.get(index + 1) == Some(&'-') {
            out.push_str(COMMENT);
            out.extend(&characters[index..]);
            out.push_str(RESET);
            break;
        }

        if character == '\'' || character == '"' {
            let end = closing(&characters, index, character);
            out.push_str(STRING);
            out.extend(&characters[index..end]);
            out.push_str(RESET);
            index = end;
            continue;
        }

        if character.is_ascii_digit() && !starts_inside_word(&characters, index) {
            let end = span(&characters, index, |c| {
                c.is_ascii_alphanumeric() || c == '.'
            });
            out.push_str(NUMBER);
            out.extend(&characters[index..end]);
            out.push_str(RESET);
            index = end;
            continue;
        }

        if character.is_ascii_alphabetic() || character == '_' {
            let end = span(&characters, index, |c| {
                c.is_ascii_alphanumeric() || c == '_'
            });
            let word: String = characters[index..end].iter().collect();
            if LOOKUP.contains(word.to_uppercase().as_str()) {
                out.push_str(KEYWORD);
                out.push_str(&word);
                out.push_str(RESET);
            } else {
                out.push_str(&word);
            }
            index = end;
            continue;
        }

        out.push(character);
        index += 1;
    }
    out
}

fn closing(characters: &[char], start: usize, quote: char) -> usize {
    let mut index = start + 1;
    while index < characters.len() {
        if characters[index] == quote {
            if characters.get(index + 1) == Some(&quote) {
                index += 2;
                continue;
            }
            return index + 1;
        }
        index += 1;
    }
    characters.len()
}

fn span(characters: &[char], start: usize, allowed: impl Fn(char) -> bool) -> usize {
    let mut index = start;
    while index < characters.len() && allowed(characters[index]) {
        index += 1;
    }
    index
}

fn starts_inside_word(characters: &[char], index: usize) -> bool {
    index > 0 && (characters[index - 1].is_ascii_alphanumeric() || characters[index - 1] == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords_are_coloured_whatever_their_case() {
        assert!(paint("select").contains(KEYWORD));
        assert!(paint("SELECT").contains(KEYWORD));
    }

    #[test]
    fn a_keyword_inside_an_identifier_stays_plain() {
        let painted = paint("selected_at");
        assert!(!painted.contains(KEYWORD));
    }

    #[test]
    fn escaped_quotes_do_not_end_the_string_early() {
        let painted = paint("'it''s' AND");
        assert!(painted.contains(STRING));
        assert!(painted.contains(KEYWORD));
    }
}
