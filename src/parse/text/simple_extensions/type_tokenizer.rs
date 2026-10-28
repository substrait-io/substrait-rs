// SPDX-License-Identifier: Apache-2.0

//! Tokenize type syntax into borrowed text and byte positions.
//! Tokens retain their source text and positions for grammar rules to consume.

use super::type_ast::TypeParseError;

/// Syntactic pieces recognized before the parser assigns them meaning.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum TokenKind {
    /// A name without a dedicated keyword token, such as `point` or `P`.
    Identifier,
    /// `func`, introducing a function type such as `func<i32 -> i64>`.
    Func,
    /// `list`, introducing a list type such as `list<i32>`.
    List,
    /// `decimal`, introducing precision and scale parameters such as `decimal<10, 2>`.
    Decimal,
    /// Bare `any`, matching an unspecified type rather than a numbered variable.
    Any,
    /// A numbered variable such as `any1` or `any12`; the parser checks its ID.
    AnyVar,
    /// Unsigned decimal digits, such as `10`; a leading sign is a separate token.
    Integer,
    /// `u!`, introducing a user-defined type such as `u!point`.
    UserDefined,
    /// `->`, separating function parameters from the return type.
    Arrow,
    /// `<`, opening a type's parameter list or function-type body.
    OpenAngle,
    /// `>`, closing a type's parameter list or function-type body.
    CloseAngle,
    /// `(`, opening grouped function parameters such as `(i32, i64)`.
    OpenParen,
    /// `)`, closing grouped function parameters.
    CloseParen,
    /// `,`, separating parameters at the current nesting level.
    Comma,
    /// `?`, marking a nullable type, as in `i32?` or `list?<i32>`.
    QuestionMark,
    /// `+`, currently accepted as an explicit sign on an integer literal.
    Plus,
    /// `-`, currently accepted as a negative literal's sign; distinct from `->`.
    Minus,
}

/// A token with its original spelling and byte offset, both retained for the parser.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Token<'a> {
    pub(super) kind: TokenKind,
    pub(super) text: &'a str,
    pub(super) position: usize,
}

/// Split source into tokens, skipping whitespace and borrowing token text.
///
/// Recognize `->` and `u!` before single-character punctuation. Whole words
/// matching `func`, `list`, `decimal`, `any`, or numbered `anyN` variables receive
/// dedicated tokens, case-insensitively. Other names remain identifiers. Original
/// spelling is retained, including for names following `u!`.
/// Digits and signs are separate tokens, and unknown characters produce an error
/// at their original byte position.
pub(super) fn tokenize(source: &str) -> Result<Vec<Token<'_>>, TypeParseError> {
    let mut tokens = Vec::new();
    let mut remaining = source.trim_start();
    while !remaining.is_empty() {
        let position = source.len() - remaining.len();
        let first = remaining.chars().next().unwrap();
        let (kind, length) = if remaining.starts_with("->") {
            (TokenKind::Arrow, 2)
        } else if remaining.starts_with("u!") {
            (TokenKind::UserDefined, 2)
        } else if first.is_ascii_alphabetic() || matches!(first, '_' | '$') {
            let length = remaining
                .find(|c: char| !c.is_ascii_alphanumeric() && !matches!(c, '_' | '$'))
                .unwrap_or(remaining.len());
            (classify_word(&remaining[..length]), length)
        } else if first.is_ascii_digit() {
            let length = remaining
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(remaining.len());
            (TokenKind::Integer, length)
        } else {
            let kind = match first {
                '<' => TokenKind::OpenAngle,
                '>' => TokenKind::CloseAngle,
                '(' => TokenKind::OpenParen,
                ')' => TokenKind::CloseParen,
                ',' => TokenKind::Comma,
                '?' => TokenKind::QuestionMark,
                '+' => TokenKind::Plus,
                '-' => TokenKind::Minus,
                '[' | ']' => return Err(TypeParseError::UnsupportedVariation(source.to_string())),
                _ => {
                    return Err(TypeParseError::UnexpectedToken {
                        position,
                        expected: "a type token",
                    });
                }
            };
            (kind, first.len_utf8())
        };
        tokens.push(Token {
            kind,
            text: &remaining[..length],
            position,
        });
        remaining = remaining[length..].trim_start();
    }
    Ok(tokens)
}

/// Classify a complete word so names like `listing` are not split into keywords.
fn classify_word(word: &str) -> TokenKind {
    match word.to_ascii_lowercase().as_str() {
        "func" => TokenKind::Func,
        "list" => TokenKind::List,
        "decimal" => TokenKind::Decimal,
        "any" => TokenKind::Any,
        name if name.strip_prefix("any").is_some_and(|suffix| {
            !suffix.is_empty() && suffix.bytes().all(|byte| byte.is_ascii_digit())
        }) =>
        {
            TokenKind::AnyVar
        }
        _ => TokenKind::Identifier,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrows_are_distinct_from_closing_brackets() {
        let tokens = tokenize("func<func<i32 -> i64> -> bool>").unwrap();
        let kinds: Vec<_> = tokens.iter().map(|token| token.kind).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Func,
                TokenKind::OpenAngle,
                TokenKind::Func,
                TokenKind::OpenAngle,
                TokenKind::Identifier,
                TokenKind::Arrow,
                TokenKind::Identifier,
                TokenKind::CloseAngle,
                TokenKind::Arrow,
                TokenKind::Identifier,
                TokenKind::CloseAngle,
            ]
        );
    }

    #[test]
    fn keywords_are_case_insensitive_and_retain_their_spelling() {
        for (source, kind) in [
            ("FuNc", TokenKind::Func),
            ("LIST", TokenKind::List),
            ("Decimal", TokenKind::Decimal),
            ("ANY", TokenKind::Any),
            ("any12", TokenKind::AnyVar),
            ("ANY2", TokenKind::AnyVar),
        ] {
            let tokens = tokenize(source).unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].kind, kind);
            assert_eq!(tokens[0].text, source);
        }
    }

    #[test]
    fn keyword_prefixes_remain_identifiers() {
        for source in [
            "function",
            "listing",
            "decimal_value",
            "anything",
            "anyfoo",
            "any1extra",
        ] {
            let tokens = tokenize(source).unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].kind, TokenKind::Identifier, "{source}");
        }
    }

    #[test]
    fn tokens_borrow_the_source_and_retain_byte_positions() {
        let tokens = tokenize("\u{2003}u!point?<i32>").unwrap();
        assert_eq!(
            tokens[0],
            Token {
                kind: TokenKind::UserDefined,
                text: "u!",
                position: 3
            }
        );
        assert_eq!(tokens[1].text, "point");
        assert_eq!(tokens[1].position, 5);
    }

    #[test]
    fn integer_signs_are_separate_from_digits() {
        let tokens = tokenize("decimal?<+10, -2>").unwrap();
        let kinds_and_text: Vec<_> = tokens
            .iter()
            .map(|token| (token.kind, token.text))
            .collect();
        assert_eq!(
            kinds_and_text,
            vec![
                (TokenKind::Decimal, "decimal"),
                (TokenKind::QuestionMark, "?"),
                (TokenKind::OpenAngle, "<"),
                (TokenKind::Plus, "+"),
                (TokenKind::Integer, "10"),
                (TokenKind::Comma, ","),
                (TokenKind::Minus, "-"),
                (TokenKind::Integer, "2"),
                (TokenKind::CloseAngle, ">"),
            ]
        );
    }

    #[test]
    fn unknown_characters_have_a_byte_position() {
        assert_eq!(
            tokenize("\u{2003}λ"),
            Err(TypeParseError::UnexpectedToken {
                position: 3,
                expected: "a type token"
            })
        );
    }
}
