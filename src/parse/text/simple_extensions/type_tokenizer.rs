// SPDX-License-Identifier: Apache-2.0

//! Tokenize type syntax into borrowed text and byte positions.
//! Grammar rules live separately in `type_parser.rs`.

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
    /// `boolean`, the Boolean type.
    Boolean,
    /// `i8`, the 8-bit integer type.
    I8,
    /// `i16`, the 16-bit integer type.
    I16,
    /// `i32`, the 32-bit integer type.
    I32,
    /// `i64`, the 64-bit integer type.
    I64,
    /// `fp32`, the 32-bit floating-point type.
    Fp32,
    /// `fp64`, the 64-bit floating-point type.
    Fp64,
    /// `string`, the string type.
    String,
    /// `binary`, the binary type.
    Binary,
    /// `date`, the date type.
    Date,
    /// `interval_year`, a year-to-month interval.
    IntervalYear,
    /// `interval_day`, a day-to-second interval.
    IntervalDay,
    /// `interval_compound`, a compound interval.
    IntervalCompound,
    /// `uuid`, the UUID type.
    Uuid,
    /// `precision_time`, a time type with a precision parameter.
    PrecisionTime,
    /// `precision_timestamp`, a timestamp type with a precision parameter.
    PrecisionTimestamp,
    /// `precision_timestamp_tz`, a timestamp-with-timezone type with a precision parameter.
    PrecisionTimestampTz,
    /// `fixedchar`, a fixed-length character type.
    FixedChar,
    /// `varchar`, a variable-length character type.
    VarChar,
    /// `fixedbinary`, a fixed-length binary type.
    FixedBinary,
    /// `struct`, an unnamed-field struct type.
    Struct,
    /// `nstruct`, the upstream named-field struct keyword.
    NStruct,
    /// `map`, a map type with key and value parameters.
    Map,
    /// `bool`, the short name for Boolean.
    Bool,
    /// `str`, the short name for string.
    Str,
    /// `vbin`, the short name for binary.
    VBin,
    /// `iyear`, the short name for a year-to-month interval.
    IYear,
    /// `iday`, the short name for a day-to-second interval.
    IDay,
    /// `icompound`, the short name for a compound interval.
    ICompound,
    /// `dec`, the short name for decimal.
    Dec,
    /// `pt`, the short name for precision_time.
    Pt,
    /// `pts`, the short name for precision_timestamp.
    Pts,
    /// `ptstz`, the short name for precision_timestamp_tz.
    PtsTz,
    /// `fchar`, the short name for fixedchar.
    FChar,
    /// `vchar`, the short name for varchar.
    VChar,
    /// `fbin`, the short name for fixedbinary.
    FBin,
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

impl TokenKind {
    /// Human-readable spelling or category for a required token.
    pub(super) fn description(self) -> &'static str {
        match self {
            Self::Identifier => "an identifier",
            Self::Func => "'func'",
            Self::List => "'list'",
            Self::Decimal => "'decimal'",
            Self::Boolean => "'boolean'",
            Self::I8 => "'i8'",
            Self::I16 => "'i16'",
            Self::I32 => "'i32'",
            Self::I64 => "'i64'",
            Self::Fp32 => "'fp32'",
            Self::Fp64 => "'fp64'",
            Self::String => "'string'",
            Self::Binary => "'binary'",
            Self::Date => "'date'",
            Self::IntervalYear => "'interval_year'",
            Self::IntervalDay => "'interval_day'",
            Self::IntervalCompound => "'interval_compound'",
            Self::Uuid => "'uuid'",
            Self::PrecisionTime => "'precision_time'",
            Self::PrecisionTimestamp => "'precision_timestamp'",
            Self::PrecisionTimestampTz => "'precision_timestamp_tz'",
            Self::FixedChar => "'fixedchar'",
            Self::VarChar => "'varchar'",
            Self::FixedBinary => "'fixedbinary'",
            Self::Struct => "'struct'",
            Self::NStruct => "'nstruct'",
            Self::Map => "'map'",
            Self::Bool => "'bool'",
            Self::Str => "'str'",
            Self::VBin => "'vbin'",
            Self::IYear => "'iyear'",
            Self::IDay => "'iday'",
            Self::ICompound => "'icompound'",
            Self::Dec => "'dec'",
            Self::Pt => "'pt'",
            Self::Pts => "'pts'",
            Self::PtsTz => "'ptstz'",
            Self::FChar => "'fchar'",
            Self::VChar => "'vchar'",
            Self::FBin => "'fbin'",
            Self::Any => "'any'",
            Self::AnyVar => "a numbered type variable",
            Self::Integer => "an integer literal",
            Self::UserDefined => "'u!'",
            Self::Arrow => "'->'",
            Self::OpenAngle => "'<'",
            Self::CloseAngle => "'>'",
            Self::OpenParen => "'('",
            Self::CloseParen => "')'",
            Self::Comma => "','",
            Self::QuestionMark => "'?'",
            Self::Plus => "'+'",
            Self::Minus => "'-'",
        }
    }

    /// Whether this token can supply a type or parameter name, including after `u!`.
    /// Recognizing a keyword does not validate or normalize the resulting type.
    pub(super) fn is_name(self) -> bool {
        matches!(
            self,
            Self::Identifier
                | Self::Func
                | Self::List
                | Self::Decimal
                | Self::Any
                | Self::AnyVar
                | Self::Boolean
                | Self::I8
                | Self::I16
                | Self::I32
                | Self::I64
                | Self::Fp32
                | Self::Fp64
                | Self::String
                | Self::Binary
                | Self::Date
                | Self::IntervalYear
                | Self::IntervalDay
                | Self::IntervalCompound
                | Self::Uuid
                | Self::PrecisionTime
                | Self::PrecisionTimestamp
                | Self::PrecisionTimestampTz
                | Self::FixedChar
                | Self::VarChar
                | Self::FixedBinary
                | Self::Struct
                | Self::NStruct
                | Self::Map
                | Self::Bool
                | Self::Str
                | Self::VBin
                | Self::IYear
                | Self::IDay
                | Self::ICompound
                | Self::Dec
                | Self::Pt
                | Self::Pts
                | Self::PtsTz
                | Self::FChar
                | Self::VChar
                | Self::FBin
        )
    }
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
/// matching upstream type keywords, short names, or numbered `anyN` variables
/// receive dedicated tokens, case-insensitively. Other names remain identifiers. Original
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
        "boolean" => TokenKind::Boolean,
        "i8" => TokenKind::I8,
        "i16" => TokenKind::I16,
        "i32" => TokenKind::I32,
        "i64" => TokenKind::I64,
        "fp32" => TokenKind::Fp32,
        "fp64" => TokenKind::Fp64,
        "string" => TokenKind::String,
        "binary" => TokenKind::Binary,
        "date" => TokenKind::Date,
        "interval_year" => TokenKind::IntervalYear,
        "interval_day" => TokenKind::IntervalDay,
        "interval_compound" => TokenKind::IntervalCompound,
        "uuid" => TokenKind::Uuid,
        "precision_time" => TokenKind::PrecisionTime,
        "precision_timestamp" => TokenKind::PrecisionTimestamp,
        "precision_timestamp_tz" => TokenKind::PrecisionTimestampTz,
        "fixedchar" => TokenKind::FixedChar,
        "varchar" => TokenKind::VarChar,
        "fixedbinary" => TokenKind::FixedBinary,
        "struct" => TokenKind::Struct,
        "nstruct" => TokenKind::NStruct,
        "map" => TokenKind::Map,
        "bool" => TokenKind::Bool,
        "str" => TokenKind::Str,
        "vbin" => TokenKind::VBin,
        "iyear" => TokenKind::IYear,
        "iday" => TokenKind::IDay,
        "icompound" => TokenKind::ICompound,
        "dec" => TokenKind::Dec,
        "pt" => TokenKind::Pt,
        "pts" => TokenKind::Pts,
        "ptstz" => TokenKind::PtsTz,
        "fchar" => TokenKind::FChar,
        "vchar" => TokenKind::VChar,
        "fbin" => TokenKind::FBin,
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
                TokenKind::I32,
                TokenKind::Arrow,
                TokenKind::I64,
                TokenKind::CloseAngle,
                TokenKind::Arrow,
                TokenKind::Bool,
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
    fn remaining_upstream_type_keywords_and_short_names_are_recognized() {
        for (source, kind) in [
            ("boolean", TokenKind::Boolean),
            ("i8", TokenKind::I8),
            ("i16", TokenKind::I16),
            ("i32", TokenKind::I32),
            ("i64", TokenKind::I64),
            ("fp32", TokenKind::Fp32),
            ("fp64", TokenKind::Fp64),
            ("string", TokenKind::String),
            ("binary", TokenKind::Binary),
            ("date", TokenKind::Date),
            ("interval_year", TokenKind::IntervalYear),
            ("interval_day", TokenKind::IntervalDay),
            ("interval_compound", TokenKind::IntervalCompound),
            ("uuid", TokenKind::Uuid),
            ("precision_time", TokenKind::PrecisionTime),
            ("precision_timestamp", TokenKind::PrecisionTimestamp),
            ("precision_timestamp_tz", TokenKind::PrecisionTimestampTz),
            ("fixedchar", TokenKind::FixedChar),
            ("varchar", TokenKind::VarChar),
            ("fixedbinary", TokenKind::FixedBinary),
            ("struct", TokenKind::Struct),
            ("nstruct", TokenKind::NStruct),
            ("map", TokenKind::Map),
            ("bool", TokenKind::Bool),
            ("str", TokenKind::Str),
            ("vbin", TokenKind::VBin),
            ("iyear", TokenKind::IYear),
            ("iday", TokenKind::IDay),
            ("icompound", TokenKind::ICompound),
            ("dec", TokenKind::Dec),
            ("pt", TokenKind::Pt),
            ("pts", TokenKind::Pts),
            ("ptstz", TokenKind::PtsTz),
            ("fchar", TokenKind::FChar),
            ("vchar", TokenKind::VChar),
            ("fbin", TokenKind::FBin),
        ] {
            for spelling in [source.to_string(), source.to_ascii_uppercase()] {
                let tokens = tokenize(&spelling).unwrap();
                assert_eq!(tokens.len(), 1);
                assert_eq!(tokens[0].kind, kind, "{spelling}");
                assert_eq!(tokens[0].text, spelling);
                assert!(kind.is_name());
            }
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
