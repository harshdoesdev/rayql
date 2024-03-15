#[derive(thiserror::Error, Debug, PartialEq)]
pub enum TokenizationError {
    #[error("Unexpected character '{char}' at line {line}, column {col}")]
    UnexpectedCharacter { char: char, line: usize, col: usize },
    #[error("String literal opened at line {line}, column {col}")]
    StringLiteralOpened { line: usize, col: usize },
    #[error("Unexpected End of Input")]
    UnexpectedEndOfInput,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    StringLiteral(String),
    Text(String),
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
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(s) => write!(f, "Identifier: {}", s),
            Token::StringLiteral(s) => write!(f, "StringLiteral: {}", s),
            Token::Text(s) => write!(f, "Text: {}", s),
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

    let mut column_number = 0;

    while let Some(ch) = chars.next() {
        column_number += 1;

        if ch == '#' && !in_string_literal {
            break;
        }

        if in_string_literal {
            if is_escaped {
                buffer.push(ch);
                is_escaped = false;
            } else if ch == '\\' {
                is_escaped = true;
                buffer.push(ch);
            } else if ch == '\'' {
                in_string_literal = false;
                tokens.push((
                    Token::StringLiteral(buffer.clone()),
                    line_number,
                    column_number - buffer.len(),
                ));
                buffer.clear();
            } else {
                buffer.push(ch);
            }
        } else if ch.is_whitespace() {
            if !buffer.is_empty() {
                tokens.push((get_token(&buffer), line_number, column_number - buffer.len()));
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
                                col: column_number,
                            });
                        }
                    } else {
                        return Err(TokenizationError::UnexpectedEndOfInput);
                    }
                }
                ch if ch.is_alphanumeric() => buffer.push(ch),
                _ => {
                    if !buffer.is_empty() {
                        tokens.push((get_token(&buffer), line_number, column_number - buffer.len()));
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
                                col: column_number,
                            });
                        }
                    };

                    tokens.push((token, line_number, column_number));
                }
            }
        }
    }

    if in_string_literal {
        return Err(TokenizationError::StringLiteralOpened {
            line: line_number,
            col: column_number,
        });
    }

    if !buffer.is_empty() {
        tokens.push((get_token(&buffer), line_number, column_number - buffer.len()));
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

    Token::Identifier(token_str.to_string())
}

// #[test]
// fn test_tokenizer() {
//     let code = r#"
// # Enum for user types
// enum user_type {
//   admin
//   developer
//   normal
//   guest
// }

// # Model declaration for 'user'
// model user {
//   id: int primary_key auto_increment,
//   username: str unique,
//   email: str unique, # this is an inline comment
//   age: int min(12),
//   is_active: bool default(false),
// }

// # Model declaration for 'post'
// model post {
//   id: int primary_key auto_increment,
//   title: str default('New Post'),
//   content: str,
//   rating: real default(-0.0),
//   author_id: int foreign_key(user.id),
//   created_at: timestamp default(now()),
// }

// # Index
// index user_post {
//     fields: user.id post.id,
//     unique: true,
// }

// # TODO: Implement these later
// # index user_post {
// #    fields: index_field(user.id, ORDER.ASC) index_field(post.id, ORDER.ASC),
// #    unique: true,
// #    type: INDEX_TYPE.BTREE,
// #    visibility: visible,
// #    include: post.title,
// #    partial: user.is_active,
// #    clustering: user.id,
// #}
// "#;

//     let tokens = rayql::schema::tokenizer::tokenize(code).unwrap();

//     assert_eq!(
//         tokens,
//         vec![
//             Token::Keyword(Keyword::Enum,),
//             Token::Identifier(String::from("user_type"),),
//             Token::BraceOpen,
//             Token::Identifier(String::from("admin"),),
//             Token::Identifier(String::from("developer"),),
//             Token::Identifier(String::from("normal"),),
//             Token::Identifier(String::from("guest"),),
//             Token::BraceClose,
//             Token::Keyword(Keyword::Model),
//             Token::Identifier(String::from("user")),
//             Token::BraceOpen,
//             Token::Identifier(String::from("id")),
//             Token::Colon,
//             Token::Keyword(Keyword::Integer),
//             Token::Keyword(Keyword::PrimaryKey),
//             Token::Keyword(Keyword::AutoIncrement),
//             Token::Comma,
//             Token::Identifier(String::from("username")),
//             Token::Colon,
//             Token::Keyword(Keyword::String),
//             Token::Keyword(Keyword::Unique),
//             Token::Comma,
//             Token::Identifier(String::from("email")),
//             Token::Colon,
//             Token::Keyword(Keyword::String),
//             Token::Keyword(Keyword::Unique),
//             Token::Comma,
//             Token::Identifier(String::from("age")),
//             Token::Colon,
//             Token::Keyword(Keyword::Integer),
//             Token::Identifier(String::from("min")),
//             Token::ParenOpen,
//             Token::Integer(12),
//             Token::ParenClose,
//             Token::Comma,
//             Token::Identifier(String::from("is_active")),
//             Token::Colon,
//             Token::Keyword(Keyword::Boolean),
//             Token::Identifier(String::from("default")),
//             Token::ParenOpen,
//             Token::Boolean(false),
//             Token::ParenClose,
//             Token::Comma,
//             Token::BraceClose,
//             Token::Keyword(Keyword::Model),
//             Token::Identifier(String::from("post")),
//             Token::BraceOpen,
//             Token::Identifier(String::from("id")),
//             Token::Colon,
//             Token::Keyword(Keyword::Integer),
//             Token::Keyword(Keyword::PrimaryKey),
//             Token::Keyword(Keyword::AutoIncrement),
//             Token::Comma,
//             Token::Identifier(String::from("title")),
//             Token::Colon,
//             Token::Keyword(Keyword::String),
//             Token::Identifier(String::from("default")),
//             Token::ParenOpen,
//             Token::StringLiteral(String::from("New Post")),
//             Token::ParenClose,
//             Token::Comma,
//             Token::Identifier(String::from("content")),
//             Token::Colon,
//             Token::Keyword(Keyword::String),
//             Token::Comma,
//             Token::Identifier(String::from("rating")),
//             Token::Colon,
//             Token::Keyword(Keyword::Real),
//             Token::Identifier(String::from("default")),
//             Token::ParenOpen,
//             Token::Real(-0.0),
//             Token::ParenClose,
//             Token::Comma,
//             Token::Identifier(String::from("author_id")),
//             Token::Colon,
//             Token::Keyword(Keyword::Integer),
//             Token::Identifier(String::from("foreign_key")),
//             Token::ParenOpen,
//             Token::Identifier(String::from("user.id")),
//             Token::ParenClose,
//             Token::Comma,
//             Token::Identifier(String::from("created_at")),
//             Token::Colon,
//             Token::Keyword(Keyword::Timestamp),
//             Token::Identifier(String::from("default")),
//             Token::ParenOpen,
//             Token::Identifier(String::from("now")),
//             Token::ParenOpen,
//             Token::ParenClose,
//             Token::ParenClose,
//             Token::Comma,
//             Token::BraceClose,
//             Token::Keyword(Keyword::Index),
//             Token::Identifier(String::from("user_post")),
//             Token::BraceOpen,
//             Token::Identifier(String::from("fields")),
//             Token::Colon,
//             Token::Identifier(String::from("user.id")),
//             Token::Identifier(String::from("post.id")),
//             Token::Comma,
//             Token::Keyword(Keyword::Unique),
//             Token::Colon,
//             Token::Boolean(true),
//             Token::Comma,
//             Token::BraceClose,
//         ]
//     );
// }
