use crate::ast::*;
use crate::parser::*;
use nom::branch::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

// -----------------------------------------------------------------------------

#[derive(Debug, Node)]
pub struct ConditionalStatement {
    pub nodes: (
        Option<UniquePriority>,
        Keyword,
        Paren<CondPredicate>,
        StatementOrNull,
        Vec<(Keyword, Keyword, Paren<CondPredicate>, StatementOrNull)>,
        Option<(Keyword, StatementOrNull)>,
    ),
}

#[derive(Debug, Node)]
pub enum UniquePriority {
    Unique(Keyword),
    Unique0(Keyword),
    Priority(Keyword),
}

#[derive(Debug, Node)]
pub struct CondPredicate {
    pub nodes: (List<Symbol, ExpressionOrCondPattern>,),
}

#[derive(Debug, Node)]
pub enum ExpressionOrCondPattern {
    Expression(Expression),
    CondPattern(CondPattern),
}

#[derive(Debug, Node)]
pub struct CondPattern {
    pub nodes: (Expression, Keyword, Pattern),
}

// -----------------------------------------------------------------------------

#[parser]
pub fn conditional_statement(s: Span) -> IResult<Span, ConditionalStatement> {
    let (s, a) = opt(unique_priority)(s)?;
    let (s, b) = keyword("if")(s)?;
    let (s, c) = paren(cond_predicate)(s)?;
    let (s, d) = statement_or_null(s)?;
    let (s, e) = many0(tuple((
        keyword("else"),
        keyword("if"),
        paren(cond_predicate),
        statement_or_null,
    )))(s)?;
    let (s, f) = opt(pair(keyword("else"), statement_or_null))(s)?;

    Ok((
        s,
        ConditionalStatement {
            nodes: (a, b, c, d, e, f),
        },
    ))
}

#[parser]
pub fn unique_priority(s: Span) -> IResult<Span, UniquePriority> {
    alt((
        map(keyword("unique0"), |x| UniquePriority::Unique0(x)),
        map(keyword("unique"), |x| UniquePriority::Unique(x)),
        map(keyword("priority"), |x| UniquePriority::Priority(x)),
    ))(s)
}

#[parser(MaybeRecursive)]
pub fn cond_predicate(s: Span) -> IResult<Span, CondPredicate> {
    let (s, a) = list(symbol("&&&"), expression_or_cond_pattern)(s)?;
    Ok((s, CondPredicate { nodes: (a,) }))
}

#[parser]
pub fn expression_or_cond_pattern(s: Span) -> IResult<Span, ExpressionOrCondPattern> {
    alt((
        map(expression, |x| ExpressionOrCondPattern::Expression(x)),
        map(cond_pattern, |x| ExpressionOrCondPattern::CondPattern(x)),
    ))(s)
}

#[parser(MaybeRecursive)]
pub fn cond_pattern(s: Span) -> IResult<Span, CondPattern> {
    let (s, a) = expression(s)?;
    let (s, b) = keyword("matches")(s)?;
    let (s, c) = pattern(s)?;
    Ok((s, CondPattern { nodes: (a, b, c) }))
}