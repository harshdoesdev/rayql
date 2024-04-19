use annotate_snippets::{Level, Renderer, Snippet};
use rayql::{
    schema::error::{ParseError, TokenizationError},
    sql::error::{FunctionError, ToSQLError},
};

struct ErrorMessageBuilder<'a> {
    code: &'a str,
    title: String,
    label: String,
    line: usize,
    span: std::ops::Range<usize>,
}

impl<'a> ErrorMessageBuilder<'a> {
    fn new(code: &'a str) -> Self {
        ErrorMessageBuilder {
            code,
            title: String::new(),
            label: String::new(),
            line: 0,
            span: 0..0,
        }
    }

    fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    fn with_label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    fn with_line(mut self, line: usize) -> Self {
        self.line = line;
        self
    }

    fn with_span(mut self, span: std::ops::Range<usize>) -> Self {
        self.span = span;
        self
    }

    fn build(self) -> String {
        let message = Level::Error.title(&self.title).snippet(
            Snippet::source(self.code.lines().nth(self.line - 1).unwrap())
                .line_start(self.line)
                .origin("schema.rayql")
                .fold(true)
                .annotation(Level::Error.span(self.span).label(&self.label)),
        );
        let renderer = Renderer::styled();
        let rendered_message = renderer.render(message);
        rendered_message.to_string()
    }
}

pub fn pretty_error_message(error: &ParseError, code: &str) -> String {
    match error {
        ParseError::TokenizationError(tokenization_error) => {
            pretty_tokenization_error_message(tokenization_error, code)
        }
        ParseError::UnexpectedToken {
            token,
            line_number,
            column,
        } => format!(
            "Unexpected token {} at line {}, column {}",
            token, line_number, column
        ),
        ParseError::InvalidReference {
            entity,
            property,
            line_number,
            column,
        } => format!(
            "Invalid Reference: Cannot access '{}' of '{}' at line {}, column {}",
            property, entity, line_number, column
        ),
        ParseError::IdentifierAlreadyInUse {
            identifier,
            line_number,
            column,
        } => format!(
            "Cannot re-define '{}' at line {}, column {}",
            identifier, line_number, column
        ),
        ParseError::UnexpectedEndOfTokens => "Unexpected end of tokens".to_string(),
        ParseError::FieldAlreadyExistsOnModel { field, model, line_number, column } => format!(
            "Field '{field}' already exists on model '{model}', cannot redeclare it at line {}, column {}.",
            line_number, column,
        ),
        ParseError::EnumVariantAlreadyExists { r#enum, variant, line_number, column } => format!(
            "Enum variant '{variant}' already exists on enum '{enum}', cannot redeclare it at line {}, column {}.",
            line_number, column,
        ),
    }
}

fn pretty_tokenization_error_message(tokenization_error: &TokenizationError, code: &str) -> String {
    match tokenization_error {
        TokenizationError::UnexpectedCharacter { char, line, column }
        | TokenizationError::UnknownEscapeSequence { char, line, column } => {
            ErrorMessageBuilder::new(code)
                .with_title("Unexpected character".to_string())
                .with_label(format!("Unexpected character '{}'", char))
                .with_line(*line)
                .with_span(*column - 1..*column)
                .build()
        }
        TokenizationError::StringLiteralOpened { line, column } => {
            format!("String literal opened at line {}, column {}", line, column)
        }
        TokenizationError::IdentifierBeginsWithDigit {
            identifier,
            line,
            column,
        } => format!(
            "Identifier '{identifier}' cannot begin with a digit at line {}, column {}",
            line, column
        ),
        TokenizationError::UnexpectedEndOfInput => "Unexpected end of input".to_string(),
    }
}

pub fn pretty_to_sql_error_message(error: ToSQLError, code: &str) -> String {
    match error {
        ToSQLError::UnknownReference {
            entity_name,
            line_number,
            column,
        } => {
            format!(
                "Unknown reference: {} at line {}, column {}",
                entity_name, line_number, column
            )
        }
        ToSQLError::EnumNotFound {
            enum_name,
            line_number,
            column,
        } => {
            format!(
                "Enum not found: {} at line {}, column {}",
                enum_name, line_number, column
            )
        }
        ToSQLError::ModelNotFound {
            model_name,
            line_number,
            column,
        } => {
            format!(
                "Model not found: {} at line {}, column {}",
                model_name, line_number, column
            )
        }
        ToSQLError::FieldNotFound {
            model_name,
            field_name,
            line_number,
            column,
        } => {
            format!(
                "Field '{}' does not exists on model '{}': at line {}, column {}",
                field_name, model_name, line_number, column
            )
        }
        ToSQLError::VariantNotFound {
            enum_name,
            variant,
            line_number,
            column,
        } => {
            format!(
                "Variant '{}' does not exists on enum '{}': at line {}, column {}",
                variant, enum_name, line_number, column
            )
        }
        ToSQLError::ConversionError {
            reason,
            line_number,
            column,
        } => {
            format!(
                "Conversion error: {} at line {}, column {}",
                reason, line_number, column
            )
        }
        ToSQLError::FunctionError {
            source,
            line_number,
            column,
        } => pretty_function_error_message(source, code, line_number, column),
    }
}

fn pretty_function_error_message(
    error: FunctionError,
    _code: &str,
    line_number: usize,
    column: usize,
) -> String {
    match error {
        FunctionError::InvalidArgument(msg) => {
            format!(
                "Invalid argument: {} at line {}, column {}",
                msg, line_number, column,
            )
        }
        FunctionError::MissingArgument => format!(
            "Missing argument at line {}, column {}",
            line_number, column,
        ),
        FunctionError::ExpectsExactlyOneArgument(func) => {
            format!(
                "{func} takes exactly one argument, error at line {}, column {}",
                line_number, column
            )
        }
        FunctionError::UndefinedFunction(func) => {
            format!(
                "Undefined function '{func}' called at line {}, column {}",
                line_number, column
            )
        }
    }
}
