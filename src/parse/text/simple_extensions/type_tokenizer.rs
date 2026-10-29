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
///
/// For `" List<i32>"`, the first token has kind `List`, text `"List"`, and position `1`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Token<'a> {
    /// The syntactic category, independent of the token's original spelling.
    pub(super) kind: TokenKind,
    /// The exact source slice.
    pub(super) text: &'a str,
    /// The starting byte offset in the full source, including skipped whitespace.
    pub(super) position: usize,
}

/// Split source into tokens, skipping whitespace and borrowing token text.
pub(super) fn tokenize(source: &str) -> Result<Vec<Token<'_>>, TypeParseError> {
    let mut tokenizer = Tokenizer {
        source,
        position: 0,
    };
    let mut tokens = Vec::new();
    while let Some(token) = tokenizer.next_token()? {
        tokens.push(token);
    }
    Ok(tokens)
}

/// A cursor over the source whose position is the next unread byte.
struct Tokenizer<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    /// Return the next character without advancing the cursor.
    fn peek(&self) -> Option<char> {
        self.source[self.position..].chars().next()
    }

    /// Advance past whitespace, stopping at the next token or the end of the source.
    fn skip_whitespace(&mut self) {
        while let Some(character) = self.peek() {
            if !character.is_whitespace() {
                break;
            }
            self.position += character.len_utf8();
        }
    }

    /// Skip whitespace and return the next token, advancing the cursor past it.
    /// Return `None` at the end of the source, or an error for unsupported input.
    fn next_token(&mut self) -> Result<Option<Token<'a>>, TypeParseError> {
        self.skip_whitespace();
        let Some(first) = self.peek() else {
            return Ok(None);
        };
        let remaining = &self.source[self.position..];
        let position = self.position;
        let (kind, length) = match first {
            '-' if remaining.starts_with("->") => (TokenKind::Arrow, 2),
            'u' if remaining.starts_with("u!") => (TokenKind::UserDefined, 2),
            'a'..='z' | 'A'..='Z' | '_' | '$' => {
                let length = remaining
                    .find(|c: char| !c.is_ascii_alphanumeric() && !matches!(c, '_' | '$'))
                    .unwrap_or(remaining.len());
                (classify_word(&remaining[..length]), length)
            }
            '0'..='9' => {
                let length = remaining
                    .find(|c: char| !c.is_ascii_digit())
                    .unwrap_or(remaining.len());
                (TokenKind::Integer, length)
            }
            '[' | ']' => {
                return Err(TypeParseError::UnsupportedVariation(
                    self.source.to_string(),
                ));
            }
            '<' => (TokenKind::OpenAngle, 1),
            '>' => (TokenKind::CloseAngle, 1),
            '(' => (TokenKind::OpenParen, 1),
            ')' => (TokenKind::CloseParen, 1),
            ',' => (TokenKind::Comma, 1),
            '?' => (TokenKind::QuestionMark, 1),
            '+' => (TokenKind::Plus, 1),
            '-' => (TokenKind::Minus, 1),
            _ => {
                return Err(TypeParseError::UnexpectedToken {
                    position,
                    expected: "a type token",
                });
            }
        };
        self.position += length;
        Ok(Some(Token {
            kind,
            text: &remaining[..length],
            position,
        }))
    }
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
    fn skip_whitespace_stops_at_the_next_token_or_end() {
        for (source, position, expected_character) in [
            ("", 0, None),
            ("i32", 0, Some('i')),
            (" \t\r\ni32", 0, Some('i')),
            (" \t", 0, None),
            ("i32 \t?", 3, Some('?')),
        ] {
            let mut tokenizer = Tokenizer { source, position };
            tokenizer.skip_whitespace();
            assert_eq!(tokenizer.peek(), expected_character, "{source:?}");
        }
    }

    #[test]
    fn next_token_advances_the_cursor_and_stops_at_end() {
        let source = "(any1, -2) -> i32? \t";
        let mut tokenizer = Tokenizer {
            source,
            position: 0,
        };
        for (kind, text, position) in [
            (TokenKind::OpenParen, "(", 0),
            (TokenKind::AnyVar, "any1", 1),
            (TokenKind::Comma, ",", 5),
            (TokenKind::Minus, "-", 7),
            (TokenKind::Integer, "2", 8),
            (TokenKind::CloseParen, ")", 9),
            (TokenKind::Arrow, "->", 11),
            (TokenKind::Identifier, "i32", 14),
            (TokenKind::QuestionMark, "?", 17),
        ] {
            assert_eq!(
                tokenizer.next_token().unwrap(),
                Some(Token {
                    kind,
                    text,
                    position
                })
            );
            assert_eq!(tokenizer.position, position + text.len());
        }
        assert_eq!(tokenizer.next_token().unwrap(), None);
        assert_eq!(tokenizer.position, source.len());
        assert_eq!(tokenizer.next_token().unwrap(), None);
        assert_eq!(tokenizer.position, source.len());
    }

    #[test]
    fn next_token_reports_an_unknown_character_after_a_token() {
        let mut tokenizer = Tokenizer {
            source: "i32 @",
            position: 0,
        };
        assert_eq!(tokenizer.next_token().unwrap().unwrap().text, "i32");
        assert_eq!(
            tokenizer.next_token(),
            Err(TypeParseError::UnexpectedToken {
                position: 4,
                expected: "a type token",
            })
        );
    }

    #[test]
    fn next_token_rejects_type_variation_brackets() {
        for source in ["i32[0]", "i32]"] {
            let mut tokenizer = Tokenizer {
                source,
                position: 0,
            };
            assert_eq!(tokenizer.next_token().unwrap().unwrap().text, "i32");
            assert_eq!(
                tokenizer.next_token(),
                Err(TypeParseError::UnsupportedVariation(source.to_string()))
            );
        }
    }

    #[test]
    fn classify_word_distinguishes_keywords_variables_and_other_names() {
        for (word, expected) in [
            ("LiSt", TokenKind::List),
            ("any", TokenKind::Any),
            ("ANY12", TokenKind::AnyVar),
            ("any1extra", TokenKind::Identifier),
            ("any_1", TokenKind::Identifier),
            ("$P1", TokenKind::Identifier),
            ("u", TokenKind::Identifier),
        ] {
            assert_eq!(classify_word(word), expected, "{word}");
        }
    }

    #[test]
    fn empty_or_whitespace_only_input_has_no_tokens() {
        for source in ["", " \t\r\n"] {
            assert!(tokenize(source).unwrap().is_empty());
        }
    }

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
        let tokens = tokenize(" u!point?<i32>").unwrap();
        assert_eq!(
            tokens[0],
            Token {
                kind: TokenKind::UserDefined,
                text: "u!",
                position: 1
            }
        );
        assert_eq!(tokens[1].text, "point");
        assert_eq!(tokens[1].position, 3);
    }

    #[test]
    fn multibyte_whitespace_advances_by_byte_length() {
        let tokens = tokenize("\u{2003}u!point?<i32>").unwrap();
        assert_eq!(tokens[0].text, "u!");
        assert_eq!(tokens[0].position, 3);
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
            tokenize(" @"),
            Err(TypeParseError::UnexpectedToken {
                position: 1,
                expected: "a type token"
            })
        );
    }
}
