use crate::ast::*;
use crate::parser::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

// -----------------------------------------------------------------------------

#[derive(Debug, Node)]
pub struct UdpInstantiation {
    pub nodes: (
        UdpIdentifier,
        Option<DriveStrength>,
        Option<Delay2>,
        List<Symbol, UdpInstance>,
        Symbol,
    ),
}

#[derive(Debug, Node)]
pub struct UdpInstance {
    pub nodes: (
        Option<NameOfInstance>,
        Paren<(
            OutputTerminal,
            Symbol,
            InputTerminal,
            Vec<(Symbol, InputTerminal)>,
        )>,
    ),
}

// -----------------------------------------------------------------------------

#[parser]
pub fn udp_instantiation(s: Span) -> IResult<Span, UdpInstantiation> {
    let (s, a) = udp_identifier(s)?;
    let (s, b) = opt(drive_strength)(s)?;
    let (s, c) = opt(delay2)(s)?;
    let (s, d) = list(symbol(","), udp_instance)(s)?;
    let (s, e) = symbol(";")(s)?;
    Ok((
        s,
        UdpInstantiation {
            nodes: (a, b, c, d, e),
        },
    ))
}

#[parser]
pub fn udp_instance(s: Span) -> IResult<Span, UdpInstance> {
    let (s, a) = opt(name_of_instance)(s)?;
    let (s, b) = paren(tuple((
        output_terminal,
        symbol(","),
        input_terminal,
        many0(pair(symbol(","), input_terminal)),
    )))(s)?;
    Ok((s, UdpInstance { nodes: (a, b) }))
}