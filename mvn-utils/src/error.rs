use nom;
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;
pub type LocatedError<'a> = nom::error::Error<Span<'a>>;
pub type LocatedIResult<'a, O> = nom::IResult<Span<'a>, O, MvnParseError<'a>>;

/*
 * Custom error type allows for domain-specific error messages,
 * located using `nom_locate` `Span`s for later annotation
 *
 *
 * From: https://iximiuz.com/en/posts/rust-writing-parsers-with-nom/
 */

#[derive(Debug, PartialEq)]
pub struct MvnParseError<'a> {
    pub span: Span<'a>,
    pub message: Option<String>,
}

impl<'a> MvnParseError<'a> {
    pub fn new(message: String, span: Span<'a>) -> Self {
        Self {
            span,
            message: Some(message),
        }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn line(&self) -> u32 {
        self.span().location_line()
    }

    pub fn offset(&self) -> usize {
        self.span().location_offset()
    }
}

// That's what makes it nom-compatible.
impl<'a> nom::error::ParseError<Span<'a>> for MvnParseError<'a> {
    fn from_error_kind(input: Span<'a>, kind: nom::error::ErrorKind) -> Self {
        Self::new(format!("parse error {:?}", kind), input)
    }

    fn append(_input: Span<'a>, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }

    fn from_char(input: Span<'a>, c: char) -> Self {
        Self::new(format!("unexpected character '{}'", c), input)
    }
}

#[macro_export]
macro_rules! error_or {
    ($result:expr, $input:expr, $message:literal) => {
        match $result {
            Ok((rest, content)) => Ok((rest, content)),
            Err(nom::Err::Error(_)) => Err(nom::Err::Error(utils::error::MvnParseError::new(
                $message.to_owned(),
                $input,
            ))),
            Err(e) => Err(e),
        }
    };
}

#[macro_export]
macro_rules! failure_or {
    ($result:expr, $input:expr, $message:literal) => {
        match $result {
            Ok((rest, content)) => Ok((rest, content)),
            Err(nom::Err::Error(_)) => Err(nom::Err::Failure(utils::error::MvnParseError::new(
                $message.to_owned(),
                $input,
            ))),
            Err(e) => Err(e),
        }
    };
}
