use crate::ast::*;
use crate::parser::*;
use nom::branch::*;
use nom::bytes::complete::*;
use nom::IResult;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Node)]
pub struct Comment {
    nodes: (Locate,),
}

// -----------------------------------------------------------------------------

#[parser]
pub(crate) fn comment(s: Span) -> IResult<Span, Comment> {
    alt((one_line_comment, block_comment))(s)
}

#[parser]
pub(crate) fn one_line_comment(s: Span) -> IResult<Span, Comment> {
    let (s, a) = tag("//")(s)?;
    let (s, b) = is_not("\n")(s)?;
    let a = concat(a, b).unwrap();
    Ok((s, Comment { nodes: (a.into(),) }))
}

#[parser]
pub(crate) fn block_comment(s: Span) -> IResult<Span, Comment> {
    let (s, a) = tag("/*")(s)?;
    let (s, b) = is_not("*/")(s)?;
    let (s, c) = tag("*/")(s)?;
    let a = concat(a, b).unwrap();
    let a = concat(a, c).unwrap();
    Ok((s, Comment { nodes: (a.into(),) }))
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use nom::combinator::*;

    #[test]
    fn test_comment() {
        parser_test!(comment, "// comment", Ok((_, _)));
        parser_test!(comment, "/* comment\n\n */", Ok((_, _)));
    }
}