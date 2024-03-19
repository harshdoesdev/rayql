use rayql::schema::error::TokenizationError;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Reference(String, String),
    StringLiteral(String),
    Integer(i64),
    Real(f64),
    Boolean(bool),
    Keyword(Keyword),
    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
    Colon,
    Comma,
    Optional(Box<Token>),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(s) => write!(f, "Identifier: {}", s),
            Token::Reference(e, p) => write!(f, "Reference: {}.{}", e, p),
            Token::StringLiteral(s) => write!(f, "StringLiteral: {}", s),
            Token::Integer(i) => write!(f, "Integer: {}", i),
            Token::Real(r) => write!(f, "Real: {}", r),
            Token::Boolean(b) => write!(f, "Boolean: {}", b),
            Token::Keyword(kw) => write!(f, "Keyword: {:?}", kw),
            Token::ParenOpen => write!(f, "("),
            Token::ParenClose => write!(f, ")"),
            Token::BraceOpen => write!(f, "{{"),
            Token::BraceClose => write!(f, "}}"),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::Optional(token) => write!(f, "Optional {}", token),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Model,
    Enum,
    Index,
    String,
    Integer,
    Real,
    Blob,
    Boolean,
    Timestamp,
    PrimaryKey,
    AutoIncrement,
    Unique,
    Required,
}

pub fn get_keyword(token_str: &str) -> Option<Keyword> {
    match token_str {
        "model" => Some(Keyword::Model),
        "enum" => Some(Keyword::Enum),
        "index" => Some(Keyword::Index),
        "str" => Some(Keyword::String),
        "int" => Some(Keyword::Integer),
        "real" => Some(Keyword::Real),
        "bool" => Some(Keyword::Boolean),
        "blob" => Some(Keyword::Blob),
        "timestamp" => Some(Keyword::Timestamp),
        "primary_key" => Some(Keyword::PrimaryKey),
        "auto_increment" => Some(Keyword::AutoIncrement),
        "unique" => Some(Keyword::Unique),
        "required" => Some(Keyword::Required),
        _ => None,
    }
}

pub fn is_comment(line: &str) -> bool {
    line.trim().starts_with('#')
}

pub fn is_boolean(token: &str) -> bool {
    matches!(token, "true" | "false")
}

pub fn tokenize(input: &str) -> Result<Vec<(Token, usize, usize)>, TokenizationError> {
    let mut tokens = Vec::new();
    for (line_num, line) in input.lines().enumerate() {
        if is_comment(line) {
            continue;
        }

        tokens.extend(tokenize_line(line, line_num + 1)?);
    }
    Ok(tokens)
}

pub fn tokenize_line(
    line: &str,
    line_number: usize,
) -> Result<Vec<(Token, usize, usize)>, TokenizationError> {
    let mut tokens = Vec::new();
    let mut in_string_literal = false;
    let mut is_escaped = false;

    let mut buffer = String::new();
    let mut chars = line.chars().peekable();

    let mut column = 0;

    while let Some(ch) = chars.next() {
        column += 1;

        if ch == '#' && !in_string_literal {
            break;
        }

        if in_string_literal {
            if is_escaped {
                match get_escape_char(&ch) {
                    Some(escape_ch) => {
                        buffer.push(escape_ch);
                        is_escaped = false;
                    }
                    None => {
                        return Err(TokenizationError::UnknownEscapeSequence {
                            char: ch,
                            line: line_number,
                            column,
                        })
                    }
                }
            } else if ch == '\\' {
                is_escaped = true;
            } else if ch == '\'' {
                in_string_literal = false;
                tokens.push((
                    Token::StringLiteral(buffer.clone()),
                    line_number,
                    column - buffer.len(),
                ));
                buffer.clear();
            } else {
                buffer.push(ch);
            }
        } else if ch.is_whitespace() {
            if !buffer.is_empty() {
                tokens.push((get_token(&buffer), line_number, column - buffer.len()));
                buffer.clear();
            }
        } else {
            match ch {
                '\'' => in_string_literal = true,
                '.' | '_' if !buffer.is_empty() => {
                    buffer.push(ch);
                }
                '-' if buffer.is_empty() => {
                    if let Some(next_char) = chars.peek() {
                        if next_char.is_numeric() || next_char.eq(&'.') {
                            buffer.push(ch);
                        } else {
                            return Err(TokenizationError::UnexpectedCharacter {
                                char: ch,
                                line: line_number,
                                column,
                            });
                        }
                    } else {
                        return Err(TokenizationError::UnexpectedEndOfInput);
                    }
                }
                '?' if !buffer.is_empty() => {
                    let token = get_token(&buffer);
                    tokens.push((
                        Token::Optional(Box::new(token)),
                        line_number,
                        column - buffer.len(),
                    ));
                    buffer.clear();
                }
                ch if ch.is_alphanumeric() => buffer.push(ch),
                _ => {
                    if !buffer.is_empty() {
                        tokens.push((get_token(&buffer), line_number, column - buffer.len()));
                        buffer.clear();
                    }

                    let token = match ch {
                        ':' => Token::Colon,
                        ',' => Token::Comma,
                        '{' => Token::BraceOpen,
                        '}' => Token::BraceClose,
                        '(' => Token::ParenOpen,
                        ')' => Token::ParenClose,
                        _ => {
                            return Err(TokenizationError::UnexpectedCharacter {
                                char: ch,
                                line: line_number,
                                column,
                            });
                        }
                    };

                    tokens.push((token, line_number, column));
                }
            }
        }
    }

    if in_string_literal {
        return Err(TokenizationError::StringLiteralOpened {
            line: line_number,
            column,
        });
    }

    if !buffer.is_empty() {
        tokens.push((get_token(&buffer), line_number, column - buffer.len()));
    }

    Ok(tokens)
}

pub fn get_token(token_str: &str) -> Token {
    if let Some(keyword) = get_keyword(token_str) {
        return Token::Keyword(keyword);
    }

    if is_boolean(token_str) {
        return Token::Boolean(token_str == "true");
    }

    if let Ok(integer) = token_str.parse::<i64>() {
        return Token::Integer(integer);
    }

    if let Ok(float) = token_str.parse::<f64>() {
        return Token::Real(float);
    }

    if let Some((entity, property)) = token_str.split_once('.') {
        return Token::Reference(entity.to_string(), property.to_string());
    }

    Token::Identifier(token_str.to_string())
}

fn get_escape_char(ch: &char) -> Option<char> {
    match ch {
        'n' => Some('\n'),
        'r' => Some('\r'),
        't' => Some('\t'),
        '\\' => Some('\\'),
        '\'' => Some('\''),
        '"' => Some('"'),
        _ => None,
    }
}
