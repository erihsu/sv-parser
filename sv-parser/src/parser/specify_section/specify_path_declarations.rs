use crate::ast::*;
use crate::parser::*;
use nom::branch::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::IResult;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Node)]
pub enum PathDeclaration {
    SimplePathDeclaration(Box<(SimplePathDeclaration, Symbol)>),
    EdgeSensitivePathDeclaration(Box<(EdgeSensitivePathDeclaration, Symbol)>),
    StateDependentPathDeclaration(Box<(StateDependentPathDeclaration, Symbol)>),
}

#[derive(Clone, Debug, Node)]
pub enum SimplePathDeclaration {
    Parallel(Box<SimplePathDeclarationParallel>),
    Full(Box<SimplePathDeclarationFull>),
}

#[derive(Clone, Debug, Node)]
pub struct SimplePathDeclarationParallel {
    pub nodes: (ParallelPathDescription, Symbol, PathDelayValue),
}

#[derive(Clone, Debug, Node)]
pub struct SimplePathDeclarationFull {
    pub nodes: (FullPathDescription, Symbol, PathDelayValue),
}

#[derive(Clone, Debug, Node)]
pub struct ParallelPathDescription {
    pub nodes: (
        Paren<(
            SpecifyInputTerminalDescriptor,
            Option<PolarityOperator>,
            Symbol,
            SpecifyOutputTerminalDescriptor,
        )>,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct FullPathDescription {
    pub nodes: (
        Paren<(
            ListOfPathInputs,
            Option<PolarityOperator>,
            Symbol,
            ListOfPathOutputs,
        )>,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct ListOfPathInputs {
    pub nodes: (List<Symbol, SpecifyInputTerminalDescriptor>,),
}

#[derive(Clone, Debug, Node)]
pub struct ListOfPathOutputs {
    pub nodes: (List<Symbol, SpecifyOutputTerminalDescriptor>,),
}

// -----------------------------------------------------------------------------

#[parser]
pub(crate) fn path_declaration(s: Span) -> IResult<Span, PathDeclaration> {
    alt((
        map(pair(simple_path_declaration, symbol(";")), |x| {
            PathDeclaration::SimplePathDeclaration(Box::new(x))
        }),
        map(pair(edge_sensitive_path_declaration, symbol(";")), |x| {
            PathDeclaration::EdgeSensitivePathDeclaration(Box::new(x))
        }),
        map(pair(state_dependent_path_declaration, symbol(";")), |x| {
            PathDeclaration::StateDependentPathDeclaration(Box::new(x))
        }),
    ))(s)
}

#[parser]
pub(crate) fn simple_path_declaration(s: Span) -> IResult<Span, SimplePathDeclaration> {
    alt((
        simple_path_declaration_parallel,
        simple_path_declaration_full,
    ))(s)
}

#[parser]
pub(crate) fn simple_path_declaration_parallel(s: Span) -> IResult<Span, SimplePathDeclaration> {
    let (s, a) = parallel_path_description(s)?;
    let (s, b) = symbol("=")(s)?;
    let (s, c) = path_delay_value(s)?;
    Ok((
        s,
        SimplePathDeclaration::Parallel(Box::new(SimplePathDeclarationParallel {
            nodes: (a, b, c),
        })),
    ))
}

#[parser]
pub(crate) fn simple_path_declaration_full(s: Span) -> IResult<Span, SimplePathDeclaration> {
    let (s, a) = full_path_description(s)?;
    let (s, b) = symbol("=")(s)?;
    let (s, c) = path_delay_value(s)?;
    Ok((
        s,
        SimplePathDeclaration::Full(Box::new(SimplePathDeclarationFull { nodes: (a, b, c) })),
    ))
}

#[parser]
pub(crate) fn parallel_path_description(s: Span) -> IResult<Span, ParallelPathDescription> {
    let (s, a) = paren(tuple((
        specify_input_terminal_descriptor,
        opt(polarity_operator),
        symbol("=>"),
        specify_output_terminal_descriptor,
    )))(s)?;
    Ok((s, ParallelPathDescription { nodes: (a,) }))
}

#[parser]
pub(crate) fn full_path_description(s: Span) -> IResult<Span, FullPathDescription> {
    let (s, a) = paren(tuple((
        list_of_path_inputs,
        opt(polarity_operator),
        symbol("*>"),
        list_of_path_outputs,
    )))(s)?;
    Ok((s, FullPathDescription { nodes: (a,) }))
}

#[parser]
pub(crate) fn list_of_path_inputs(s: Span) -> IResult<Span, ListOfPathInputs> {
    let (s, a) = list(symbol(","), specify_input_terminal_descriptor)(s)?;
    Ok((s, ListOfPathInputs { nodes: (a,) }))
}

#[parser]
pub(crate) fn list_of_path_outputs(s: Span) -> IResult<Span, ListOfPathOutputs> {
    let (s, a) = list(symbol(","), specify_output_terminal_descriptor)(s)?;
    Ok((s, ListOfPathOutputs { nodes: (a,) }))
}