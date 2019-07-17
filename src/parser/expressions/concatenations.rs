use crate::ast::*;
use crate::parser::*;
use nom::branch::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::IResult;

// -----------------------------------------------------------------------------

#[derive(Debug, Node)]
pub struct Concatenation<'a> {
    pub nodes: (Brace<'a, List<Symbol<'a>, Expression<'a>>>,),
}

#[derive(Debug, Node)]
pub struct ConstantConcatenation<'a> {
    pub nodes: (Brace<'a, List<Symbol<'a>, ConstantExpression<'a>>>,),
}

#[derive(Debug, Node)]
pub struct ConstantMultipleConcatenation<'a> {
    pub nodes: (Brace<'a, (ConstantExpression<'a>, ConstantConcatenation<'a>)>,),
}

#[derive(Debug, Node)]
pub struct ModulePathConcatenation<'a> {
    pub nodes: (Brace<'a, List<Symbol<'a>, ModulePathExpression<'a>>>,),
}

#[derive(Debug, Node)]
pub struct ModulePathMultipleConcatenation<'a> {
    pub nodes: (Brace<'a, (ConstantExpression<'a>, ModulePathConcatenation<'a>)>,),
}

#[derive(Debug, Node)]
pub struct MultipleConcatenation<'a> {
    pub nodes: (Brace<'a, (Expression<'a>, Concatenation<'a>)>,),
}

#[derive(Debug, Node)]
pub struct StreamingConcatenation<'a> {
    pub nodes: (
        Brace<
            'a,
            (
                StreamOperator<'a>,
                Option<SliceSize<'a>>,
                StreamConcatenation<'a>,
            ),
        >,
    ),
}

#[derive(Debug, Node)]
pub struct StreamOperator<'a> {
    pub nodes: (Symbol<'a>,),
}

#[derive(Debug, Node)]
pub enum SliceSize<'a> {
    SimpleType(SimpleType<'a>),
    ConstantExpression(ConstantExpression<'a>),
}

#[derive(Debug, Node)]
pub struct StreamConcatenation<'a> {
    pub nodes: (Brace<'a, List<Symbol<'a>, StreamExpression<'a>>>,),
}

#[derive(Debug, Node)]
pub struct StreamExpression<'a> {
    pub nodes: (
        Expression<'a>,
        Option<(Symbol<'a>, Bracket<'a, ArrayRangeExpression<'a>>)>,
    ),
}

#[derive(Debug, Node)]
pub enum ArrayRangeExpression<'a> {
    Expression(Expression<'a>),
    Colon(ArrayRangeExpressionColon<'a>),
    PlusColon(ArrayRangeExpressionPlusColon<'a>),
    MinusColon(ArrayRangeExpressionMinusColon<'a>),
}

#[derive(Debug, Node)]
pub struct ArrayRangeExpressionColon<'a> {
    pub nodes: (Expression<'a>, Symbol<'a>, Expression<'a>),
}

#[derive(Debug, Node)]
pub struct ArrayRangeExpressionPlusColon<'a> {
    pub nodes: (Expression<'a>, Symbol<'a>, Expression<'a>),
}

#[derive(Debug, Node)]
pub struct ArrayRangeExpressionMinusColon<'a> {
    pub nodes: (Expression<'a>, Symbol<'a>, Expression<'a>),
}

#[derive(Debug, Node)]
pub struct EmptyUnpackedArrayConcatenation<'a> {
    pub nodes: (Symbol<'a>, Symbol<'a>),
}

// -----------------------------------------------------------------------------

#[parser]
pub fn concatenation(s: Span) -> IResult<Span, Concatenation> {
    let (s, a) = brace(list(symbol(","), expression))(s)?;
    Ok((s, Concatenation { nodes: (a,) }))
}

#[parser]
pub fn constant_concatenation(s: Span) -> IResult<Span, ConstantConcatenation> {
    let (s, a) = brace(list(symbol(","), constant_expression))(s)?;
    Ok((s, ConstantConcatenation { nodes: (a,) }))
}

#[parser]
pub fn constant_multiple_concatenation(s: Span) -> IResult<Span, ConstantMultipleConcatenation> {
    let (s, a) = brace(pair(constant_expression, constant_concatenation))(s)?;
    Ok((s, ConstantMultipleConcatenation { nodes: (a,) }))
}

#[parser]
pub fn module_path_concatenation(s: Span) -> IResult<Span, ModulePathConcatenation> {
    let (s, a) = brace(list(symbol(","), module_path_expression))(s)?;
    Ok((s, ModulePathConcatenation { nodes: (a,) }))
}

#[parser]
pub fn module_path_multiple_concatenation(
    s: Span,
) -> IResult<Span, ModulePathMultipleConcatenation> {
    let (s, a) = brace(pair(constant_expression, module_path_concatenation))(s)?;
    Ok((s, ModulePathMultipleConcatenation { nodes: (a,) }))
}

#[parser]
pub fn multiple_concatenation(s: Span) -> IResult<Span, MultipleConcatenation> {
    let (s, a) = brace(pair(expression, concatenation))(s)?;
    Ok((s, MultipleConcatenation { nodes: (a,) }))
}

#[parser]
pub fn streaming_concatenation(s: Span) -> IResult<Span, StreamingConcatenation> {
    let (s, a) = brace(triple(
        stream_operator,
        opt(slice_size),
        stream_concatenation,
    ))(s)?;
    Ok((s, StreamingConcatenation { nodes: (a,) }))
}

#[parser]
pub fn stream_operator(s: Span) -> IResult<Span, StreamOperator> {
    alt((
        map(symbol(">>"), |x| StreamOperator { nodes: (x,) }),
        map(symbol("<<"), |x| StreamOperator { nodes: (x,) }),
    ))(s)
}

#[parser]
pub fn slice_size(s: Span) -> IResult<Span, SliceSize> {
    alt((
        map(simple_type, |x| SliceSize::SimpleType(x)),
        map(constant_expression, |x| SliceSize::ConstantExpression(x)),
    ))(s)
}

#[parser]
pub fn stream_concatenation(s: Span) -> IResult<Span, StreamConcatenation> {
    let (s, a) = brace(list(symbol(","), stream_expression))(s)?;
    Ok((s, StreamConcatenation { nodes: (a,) }))
}

#[parser]
pub fn stream_expression(s: Span) -> IResult<Span, StreamExpression> {
    let (s, a) = expression(s)?;
    let (s, b) = opt(pair(symbol("with"), bracket(array_range_expression)))(s)?;
    Ok((s, StreamExpression { nodes: (a, b) }))
}

#[parser]
pub fn array_range_expression(s: Span) -> IResult<Span, ArrayRangeExpression> {
    alt((
        map(expression, |x| ArrayRangeExpression::Expression(x)),
        array_range_expression_colon,
        array_range_expression_plus_colon,
        array_range_expression_minus_colon,
    ))(s)
}

#[parser]
pub fn array_range_expression_colon(s: Span) -> IResult<Span, ArrayRangeExpression> {
    let (s, a) = expression(s)?;
    let (s, b) = symbol(":")(s)?;
    let (s, c) = expression(s)?;
    Ok((
        s,
        ArrayRangeExpression::Colon(ArrayRangeExpressionColon { nodes: (a, b, c) }),
    ))
}

#[parser]
pub fn array_range_expression_plus_colon(s: Span) -> IResult<Span, ArrayRangeExpression> {
    let (s, a) = expression(s)?;
    let (s, b) = symbol("+:")(s)?;
    let (s, c) = expression(s)?;
    Ok((
        s,
        ArrayRangeExpression::PlusColon(ArrayRangeExpressionPlusColon { nodes: (a, b, c) }),
    ))
}

#[parser]
pub fn array_range_expression_minus_colon(s: Span) -> IResult<Span, ArrayRangeExpression> {
    let (s, a) = expression(s)?;
    let (s, b) = symbol("-:")(s)?;
    let (s, c) = expression(s)?;
    Ok((
        s,
        ArrayRangeExpression::MinusColon(ArrayRangeExpressionMinusColon { nodes: (a, b, c) }),
    ))
}

#[parser]
pub fn empty_unpacked_array_concatenation(
    s: Span,
) -> IResult<Span, EmptyUnpackedArrayConcatenation> {
    let (s, a) = symbol("{")(s)?;
    let (s, b) = symbol("}")(s)?;
    Ok((s, EmptyUnpackedArrayConcatenation { nodes: (a, b) }))
}

// -----------------------------------------------------------------------------
