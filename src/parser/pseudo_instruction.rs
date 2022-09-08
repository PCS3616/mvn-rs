/*
 * @ (Operando numérico: define endereço da instrução seguinte)
 * $ (Reserva de área de dados)
 * # (Final físico do texto-fonte. Operando=endereço de execução)
 * K (Constante. Operando numérico = valor da constante, em hexadecimal) 
 *
 * &   –   Origem relocável
 * >   –   Endereço simbólico de entrada (entry point)
 * <   –   Endereço simbólico externo (external)
 */

use nom::{branch::alt, combinator::value, bytes::complete::tag, IResult};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PseudoInstruction {
    AbsolutePosition,
    EmptySpace,
    FontCodeEnd,
    Constant,

    RelativePosition,
    EntryPointAddress,
    ExternalAddress,
}

impl PseudoInstruction {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::AbsolutePosition, tag(Self::AbsolutePosition.to_str())),
            value(Self::EmptySpace, tag(Self::EmptySpace.to_str())),
            value(Self::FontCodeEnd, tag(Self::FontCodeEnd.to_str())),
            value(Self::Constant, tag(Self::Constant.to_str())),
            value(Self::RelativePosition, tag(Self::RelativePosition.to_str())),
            value(Self::EntryPointAddress, tag(Self::EntryPointAddress.to_str())),
            value(Self::ExternalAddress, tag(Self::ExternalAddress.to_str())),
        ))(input)
    }
    
    pub fn to_str(&self) -> &str {
        match self {
            Self::AbsolutePosition => "@",
            Self::EmptySpace => "$",
            Self::FontCodeEnd => "#",
            Self::Constant => "K",
            Self::RelativePosition => "&",
            Self::EntryPointAddress => ">",
            Self::ExternalAddress => "<",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_pseudo_intruction() {
        assert_eq!(PseudoInstruction::parse("@"), Ok(("", PseudoInstruction::AbsolutePosition)));
        assert_eq!(PseudoInstruction::parse("$"), Ok(("", PseudoInstruction::EmptySpace)));
        assert_eq!(PseudoInstruction::parse("#"), Ok(("", PseudoInstruction::FontCodeEnd)));
        assert_eq!(PseudoInstruction::parse("K"), Ok(("", PseudoInstruction::Constant)));
        assert_eq!(PseudoInstruction::parse("&"), Ok(("", PseudoInstruction::RelativePosition)));
        assert_eq!(PseudoInstruction::parse(">"), Ok(("", PseudoInstruction::EntryPointAddress)));
        assert_eq!(PseudoInstruction::parse("<"), Ok(("", PseudoInstruction::ExternalAddress)));
    }
}

