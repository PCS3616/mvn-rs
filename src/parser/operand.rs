use nom::branch::alt;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::char;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::multi::many0_count;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;
use nom::multi::many0;
use nom::character::complete::one_of;
use nom::multi::many1;
use nom::combinator::recognize;
use nom::bytes::complete::tag;

use super::label::Label;

#[derive(Debug, PartialEq)]
pub enum Operand<'a> {
    Simbolic(Label<'a>),
    Numeric(u16),
}

impl<'a> Operand<'a> {
    fn parse(input: &'a str) -> IResult<&str, Operand<'a>> {
        alt((
            // hexa
            map_res(
                preceded(
                    tag("/"),
                    recognize(
                      many1(
                        terminated(one_of("0123456789abcdefABCDEF"), many0(char('_')))
                      )
                    )
                  ),
                |out: &str| u16::from_str_radix(&str::replace(&out, "_", ""), 16).map(Self::new_numeric)
                ),
            // label
                map(
              recognize(
                pair(
                  alt((alpha1, tag("_"))),
                  many0_count(alt((alphanumeric1, tag("_"))))
                )),
                |out: &str| Self::new_simbolic(Label::new(out))
            )
          ))(input)
    }

    fn new_numeric(value: u16) -> Self {
        Operand::Numeric(value)
    }
    
    fn new_simbolic(label: Label<'a>) -> Self {
        Operand::Simbolic(label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_numeric() {
        assert_eq!(Operand::parse("/000F"), Ok(("", Operand::Numeric(15))));
        assert_eq!(Operand::parse("/F"), Ok(("", Operand::Numeric(15))));
    }

    #[test]
    fn should_parse_simbolic() {
        assert_eq!(Operand::parse("label"), Ok(("", Operand::Simbolic(Label::new("label")))));
        assert!(Operand::parse("1label").is_err());
    }
}

