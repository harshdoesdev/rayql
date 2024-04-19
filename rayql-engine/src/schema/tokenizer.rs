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

impl Token {
    pub fn len(&self) -> usize {
        match self {
            Token::Identifier(s) => s.len(),
            Token::Reference(e, p) => e.len() + p.len() + 1, // +1 for the dot
            Token::StringLiteral(s) => s.len(),
            Token::Integer(i) => i.to_string().len(),
            Token::Real(r) => r.to_string().len(),
            Token::Boolean(b) => b.to_string().len(),
            Token::Keyword(kw) => format!("{:?}", kw).len(),
            Token::ParenOpen => 1,
            Token::ParenClose => 1,
            Token::BraceOpen => 1,
            Token::BraceClose => 1,
            Token::Colon => 1,
            Token::Comma => 1,
            Token::Optional(token) => token.len() + 9, // +9 for "Optional " prefix
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
        _ => None,
    }
}

pub fn is_comment(line: &str) -> bool {
    line.trim().starts_with('#')
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
                let escape_ch = match get_escape_char(&ch) {
                    Some(escape_ch) => escape_ch,
                    None => {
                        return Err(TokenizationError::UnknownEscapeSequence {
                            char: ch,
                            line: line_number,
                            column,
                        })
                    }
                };

                buffer.push(escape_ch);
                is_escaped = false;
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
                tokens.push((
                    get_token(&buffer, line_number, column)?,
                    line_number,
                    column - buffer.len(),
                ));
                buffer.clear();
            }
        } else {
            match ch {
                '\'' if buffer.is_empty() => in_string_literal = true,
                '_' if !buffer.is_empty() => {
                    if !is_valid_identifier(&buffer) {
                        return Err(TokenizationError::UnexpectedCharacter {
                            char: ch,
                            line: line_number,
                            column,
                        });
                    }

                    buffer.push(ch);
                }
                '.' => match chars.peek() {
                    Some(next_char) => {
                        if (!buffer.is_empty() && next_char.is_alphanumeric())
                            || next_char.is_numeric()
                        {
                            buffer.push(ch);
                        } else if next_char.is_whitespace() {
                            return Err(TokenizationError::UnexpectedCharacter {
                                char: ch,
                                line: line_number,
                                column,
                            });
                        } else {
                            return Err(TokenizationError::UnexpectedCharacter {
                                char: *next_char,
                                line: line_number,
                                column,
                            });
                        }
                    }
                    None => return Err(TokenizationError::UnexpectedEndOfInput),
                },
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
                '-' if buffer.ends_with('e') => {
                    if let Some(next_char) = chars.peek() {
                        if next_char.is_numeric() {
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
                    let token = get_token(&buffer, line_number, column)?;
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
                        tokens.push((
                            get_token(&buffer, line_number, column)?,
                            line_number,
                            column - buffer.len(),
                        ));
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
        tokens.push((
            get_token(&buffer, line_number, column)?,
            line_number,
            column - buffer.len(),
        ));
    }

    Ok(tokens)
}

pub fn get_token(
    token_str: &str,
    line_number: usize,
    column: usize,
) -> Result<Token, TokenizationError> {
    if let Some(keyword) = get_keyword(token_str) {
        return Ok(Token::Keyword(keyword));
    }

    if let Ok(boolean) = token_str.parse::<bool>() {
        return Ok(Token::Boolean(boolean));
    }

    if let Ok(integer) = token_str.parse::<i64>() {
        return Ok(Token::Integer(integer));
    }

    if let Ok(float) = token_str.parse::<f64>() {
        return Ok(Token::Real(float));
    }

    if let Some((base, exponent)) = token_str.split_once('e') {
        if let (Ok(base), Ok(exponent)) = (base.parse::<f64>(), exponent.parse::<i32>()) {
            return Ok(Token::Real(base * 10f64.powi(exponent)));
        }
    }

    if let Some((entity, property)) = token_str.split_once('.') {
        if !is_valid_identifier(entity) {
            return Err(TokenizationError::IdentifierBeginsWithDigit {
                identifier: entity.to_string(),
                line: line_number,
                column,
            });
        }

        return Ok(Token::Reference(entity.to_string(), property.to_string()));
    }

    if is_valid_identifier(token_str) {
        Ok(Token::Identifier(token_str.to_string()))
    } else {
        Err(TokenizationError::IdentifierBeginsWithDigit {
            identifier: token_str.to_string(),
            line: line_number,
            column,
        })
    }
}

fn is_valid_identifier(identifier: &str) -> bool {
    if identifier.chars().next().unwrap().is_ascii_digit() {
        return false;
    }

    true
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
