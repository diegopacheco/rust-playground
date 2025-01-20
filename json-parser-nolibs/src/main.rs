#[derive(Debug)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(std::collections::HashMap<String, JsonValue>),
}

pub fn parse_json(input: &str) -> Result<JsonValue, String> {
    let mut chars = input.chars().peekable();
    parse_value(&mut chars)
}

fn eat_whitespace<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) {
    while let Some(c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

fn parse_value<I: Iterator<Item = char>>(
    chars: &mut std::iter::Peekable<I>,
) -> Result<JsonValue, String> {
    eat_whitespace(chars);
    if let Some(&ch) = chars.peek() {
        match ch {
            '{' => parse_object(chars),
            '[' => parse_array(chars),
            '"' => parse_string(chars).map(JsonValue::String),
            't' | 'f' => parse_bool(chars),
            'n' => parse_null(chars),
            '-' | '0'..='9' => parse_number(chars),
            _ => Err(format!("Unexpected character: {}", ch)),
        }
    } else {
        Err("Unexpected end of input".to_string())
    }
}

fn parse_object<I: Iterator<Item = char>>(
    chars: &mut std::iter::Peekable<I>,
) -> Result<JsonValue, String> {
    let mut map = std::collections::HashMap::new();
    chars.next(); // consume '{'
    loop {
        eat_whitespace(chars);
        if let Some(&'}') = chars.peek() {
            chars.next(); // consume '}'
            break;
        }
        // Parse key
        let key = parse_string(chars)?;
        eat_whitespace(chars);
        if chars.next() != Some(':') {
            return Err("Expected ':' in object".to_string());
        }
        // Parse value
        let value = parse_value(chars)?;
        map.insert(key, value);
        eat_whitespace(chars);
        if let Some(&'}') = chars.peek() {
            chars.next();
            break;
        }
        if chars.next() != Some(',') {
            return Err("Expected ',' or '}' in object".to_string());
        }
    }
    Ok(JsonValue::Object(map))
}

fn parse_array<I: Iterator<Item = char>>(
    chars: &mut std::iter::Peekable<I>,
) -> Result<JsonValue, String> {
    let mut arr = Vec::new();
    chars.next(); // consume '['
    loop {
        eat_whitespace(chars);
        if let Some(&']') = chars.peek() {
            chars.next(); // consume ']'
            break;
        }
        arr.push(parse_value(chars)?);
        eat_whitespace(chars);
        if let Some(&']') = chars.peek() {
            chars.next();
            break;
        }
        if chars.next() != Some(',') {
            return Err("Expected ',' or ']' in array".to_string());
        }
    }
    Ok(JsonValue::Array(arr))
}

fn parse_string<I: Iterator<Item = char>>(
    chars: &mut std::iter::Peekable<I>,
) -> Result<String, String> {
    let mut result = String::new();
    if chars.next() != Some('"') {
        return Err("Expected '\"' at start of string".to_string());
    }
    while let Some(ch) = chars.next() {
        match ch {
            '"' => return Ok(result),
            '\\' => {
                if let Some(escaped) = chars.next() {
                    match escaped {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        _ => return Err(format!("Invalid escape sequence: \\{}", escaped)),
                    }
                } else {
                    return Err("Unexpected end in escape sequence".to_string());
                }
            }
            _ => result.push(ch),
        }
    }
    Err("Unclosed string literal".to_string())
}

fn parse_bool<I: Iterator<Item = char>>(
    chars: &mut std::iter::Peekable<I>,
) -> Result<JsonValue, String> {
    let mut buf = String::new();
    for _ in 0..5 {
        if let Some(&ch) = chars.peek() {
            if ch.is_alphabetic() {
                buf.push(ch);
                chars.next();
            } else {
                break;
            }
        }
    }
    match buf.as_str() {
        "true" => Ok(JsonValue::Bool(true)),
        "false" => Ok(JsonValue::Bool(false)),
        _ => Err(format!("Invalid boolean: {}", buf)),
    }
}

fn parse_null<I: Iterator<Item = char>>(
    chars: &mut std::iter::Peekable<I>,
) -> Result<JsonValue, String> {
    let mut buf = String::new();
    for _ in 0..4 {
        if let Some(&ch) = chars.peek() {
            buf.push(ch);
            chars.next();
        }
    }
    if buf == "null" {
        Ok(JsonValue::Null)
    } else {
        Err(format!("Invalid null: {}", buf))
    }
}

fn parse_number<I: Iterator<Item = char>>(
    chars: &mut std::iter::Peekable<I>,
) -> Result<JsonValue, String> {
    let mut num_str = String::new();
    if let Some(&ch) = chars.peek() {
        if ch == '-' {
            num_str.push(ch);
            chars.next();
        }
    }
    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' | '.' => {
                num_str.push(ch);
                chars.next();
            }
            _ => break,
        }
    }
    if let Ok(num) = num_str.parse::<f64>() {
        Ok(JsonValue::Number(num))
    } else {
        Err(format!("Invalid number: {}", num_str))
    }
}

fn main() {
    let example = r#" { "foo": [1, 2, 3], "bar": true } "#;
    match parse_json(example) {
        Ok(value) => println!("{:?}", value),
        Err(e) => eprintln!("Error: {}", e),
    }
}
