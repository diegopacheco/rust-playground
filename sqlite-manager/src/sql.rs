use rusqlite::types::ValueRef;

pub fn quote_ident(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}

pub fn literal(value: ValueRef<'_>) -> String {
    match value {
        ValueRef::Null => "NULL".to_string(),
        ValueRef::Integer(number) => number.to_string(),
        ValueRef::Real(number) => real(number),
        ValueRef::Text(bytes) => match std::str::from_utf8(bytes) {
            Ok(text) => format!("'{}'", text.replace('\'', "''")),
            Err(_) => format!("CAST({} AS TEXT)", blob(bytes)),
        },
        ValueRef::Blob(bytes) => blob(bytes),
    }
}

fn real(value: f64) -> String {
    if value.is_nan() {
        return "NULL".to_string();
    }
    if value.is_infinite() {
        return if value.is_sign_positive() {
            "9e999"
        } else {
            "-9e999"
        }
        .to_string();
    }
    format!("{value:?}")
}

fn blob(bytes: &[u8]) -> String {
    let mut text = String::with_capacity(bytes.len() * 2 + 3);
    text.push_str("X'");
    for byte in bytes {
        text.push_str(&format!("{byte:02X}"));
    }
    text.push('\'');
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers_survive_embedded_quotes() {
        assert_eq!(quote_ident("order"), "\"order\"");
        assert_eq!(quote_ident("we\"ird"), "\"we\"\"ird\"");
    }

    #[test]
    fn text_quotes_are_doubled_so_the_statement_stays_parseable() {
        assert_eq!(literal(ValueRef::Text(b"it's")), "'it''s'");
    }

    #[test]
    fn invalid_utf8_text_is_kept_as_a_casted_blob_rather_than_lost() {
        assert_eq!(
            literal(ValueRef::Text(&[0xff, 0xfe])),
            "CAST(X'FFFE' AS TEXT)"
        );
    }

    #[test]
    fn blobs_round_trip_as_hex() {
        assert_eq!(literal(ValueRef::Blob(&[0xde, 0xad])), "X'DEAD'");
        assert_eq!(literal(ValueRef::Blob(&[])), "X''");
    }

    #[test]
    fn reals_keep_full_precision_so_a_restore_matches() {
        assert_eq!(literal(ValueRef::Real(1e300)), "1e300");
        assert_eq!(literal(ValueRef::Real(0.1)), "0.1");
        assert_eq!(literal(ValueRef::Real(2.0)), "2.0");
    }

    #[test]
    fn non_finite_reals_use_values_sqlite_can_parse_back() {
        assert_eq!(literal(ValueRef::Real(f64::INFINITY)), "9e999");
        assert_eq!(literal(ValueRef::Real(f64::NEG_INFINITY)), "-9e999");
        assert_eq!(literal(ValueRef::Real(f64::NAN)), "NULL");
    }

    #[test]
    fn null_is_emitted_unquoted() {
        assert_eq!(literal(ValueRef::Null), "NULL");
    }
}
