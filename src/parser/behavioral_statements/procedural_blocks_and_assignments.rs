use crate::ast::*;
use crate::parser::*;
use nom::branch::*;
use nom::combinator::*;
use nom::IResult;

// -----------------------------------------------------------------------------

#[derive(Debug, Node)]
pub struct InitialConstruct {
    pub nodes: (Keyword, StatementOrNull),
}

#[derive(Debug, Node)]
pub struct AlwaysConstruct {
    pub nodes: (AlwaysKeyword, Statement),
}

#[derive(Debug, Node)]
pub enum AlwaysKeyword {
    Always(Keyword),
    AlwaysComb(Keyword),
    AlwaysLatch(Keyword),
    AlwaysFf(Keyword),
}

#[derive(Debug, Node)]
pub struct FinalConstruct {
    pub nodes: (Keyword, FunctionStatement),
}

#[derive(Debug, Node)]
pub enum BlockingAssignment {
    Variable(BlockingAssignmentVariable),
    NonrangeVariable(BlockingAssignmentNonrangeVariable),
    HierarchicalVariable(BlockingAssignmentHierarchicalVariable),
    OperatorAssignment(OperatorAssignment),
}

#[derive(Debug, Node)]
pub struct BlockingAssignmentVariable {
    pub nodes: (
        VariableLvalue,
        Symbol,
        DelayOrEventControl,
        Expression,
    ),
}

#[derive(Debug, Node)]
pub struct BlockingAssignmentNonrangeVariable {
    pub nodes: (NonrangeVariableLvalue, Symbol, DynamicArrayNew),
}

#[derive(Debug, Node)]
pub struct BlockingAssignmentHierarchicalVariable {
    pub nodes: (
        Option<ImplicitClassHandleOrClassScopeOrPackageScope>,
        HierarchicalVariableIdentifier,
        Select,
        Symbol,
        ClassNew,
    ),
}

#[derive(Debug, Node)]
pub struct OperatorAssignment {
    pub nodes: (VariableLvalue, AssignmentOperator, Expression),
}

#[derive(Debug, Node)]
pub struct AssignmentOperator {
    pub nodes: (Symbol,),
}

#[derive(Debug, Node)]
pub struct NonblockingAssignment {
    pub nodes: (
        VariableLvalue,
        Symbol,
        Option<DelayOrEventControl>,
        Expression,
    ),
}

#[derive(Debug, Node)]
pub enum ProceduralContinuousAssignment {
    Assign(ProceduralContinuousAssignmentAssign),
    Deassign(ProceduralContinuousAssignmentDeassign),
    ForceVariable(ProceduralContinuousAssignmentForceVariable),
    ForceNet(ProceduralContinuousAssignmentForceNet),
    ReleaseVariable(ProceduralContinuousAssignmentReleaseVariable),
    ReleaseNet(ProceduralContinuousAssignmentReleaseNet),
}

#[derive(Debug, Node)]
pub struct ProceduralContinuousAssignmentAssign {
    pub nodes: (Keyword, VariableAssignment),
}

#[derive(Debug, Node)]
pub struct ProceduralContinuousAssignmentDeassign {
    pub nodes: (Keyword, VariableLvalue),
}

#[derive(Debug, Node)]
pub struct ProceduralContinuousAssignmentForceVariable {
    pub nodes: (Keyword, VariableAssignment),
}

#[derive(Debug, Node)]
pub struct ProceduralContinuousAssignmentForceNet {
    pub nodes: (Keyword, NetAssignment),
}

#[derive(Debug, Node)]
pub struct ProceduralContinuousAssignmentReleaseVariable {
    pub nodes: (Keyword, VariableLvalue),
}

#[derive(Debug, Node)]
pub struct ProceduralContinuousAssignmentReleaseNet {
    pub nodes: (Keyword, NetLvalue),
}

#[derive(Debug, Node)]
pub struct VariableAssignment {
    pub nodes: (VariableLvalue, Symbol, Expression),
}

// -----------------------------------------------------------------------------

#[parser]
pub fn initial_construct(s: Span) -> IResult<Span, InitialConstruct> {
    let (s, a) = keyword("initial")(s)?;
    let (s, b) = statement_or_null(s)?;
    Ok((s, InitialConstruct { nodes: (a, b) }))
}

#[parser]
pub fn always_construct(s: Span) -> IResult<Span, AlwaysConstruct> {
    let (s, a) = always_keyword(s)?;
    let (s, b) = statement(s)?;
    Ok((s, AlwaysConstruct { nodes: (a, b) }))
}

#[parser]
pub fn always_keyword(s: Span) -> IResult<Span, AlwaysKeyword> {
    alt((
        map(keyword("always_comb"), |x| AlwaysKeyword::AlwaysComb(x)),
        map(keyword("always_latch"), |x| AlwaysKeyword::AlwaysLatch(x)),
        map(keyword("always_ff"), |x| AlwaysKeyword::AlwaysFf(x)),
        map(keyword("always"), |x| AlwaysKeyword::Always(x)),
    ))(s)
}

#[parser]
pub fn final_construct(s: Span) -> IResult<Span, FinalConstruct> {
    let (s, a) = keyword("final")(s)?;
    let (s, b) = function_statement(s)?;
    Ok((s, FinalConstruct { nodes: (a, b) }))
}

#[parser]
pub fn blocking_assignment(s: Span) -> IResult<Span, BlockingAssignment> {
    alt((
        blocking_assignment_variable,
        blocking_assignment_nonrange_variable,
        blocking_assignment_hierarchical_variable,
        map(operator_assignment, |x| {
            BlockingAssignment::OperatorAssignment(x)
        }),
    ))(s)
}

#[parser(MaybeRecursive)]
pub fn blocking_assignment_variable(s: Span) -> IResult<Span, BlockingAssignment> {
    let (s, a) = variable_lvalue(s)?;
    let (s, b) = symbol("=")(s)?;
    let (s, c) = delay_or_event_control(s)?;
    let (s, d) = expression(s)?;
    Ok((
        s,
        BlockingAssignment::Variable(BlockingAssignmentVariable {
            nodes: (a, b, c, d),
        }),
    ))
}

#[parser(MaybeRecursive)]
pub fn blocking_assignment_nonrange_variable(s: Span) -> IResult<Span, BlockingAssignment> {
    let (s, a) = nonrange_variable_lvalue(s)?;
    let (s, b) = symbol("=")(s)?;
    let (s, c) = dynamic_array_new(s)?;
    Ok((
        s,
        BlockingAssignment::NonrangeVariable(BlockingAssignmentNonrangeVariable {
            nodes: (a, b, c),
        }),
    ))
}

#[parser]
pub fn blocking_assignment_hierarchical_variable(s: Span) -> IResult<Span, BlockingAssignment> {
    let (s, a) = opt(implicit_class_handle_or_class_scope_or_package_scope)(s)?;
    let (s, b) = hierarchical_variable_identifier(s)?;
    let (s, c) = select(s)?;
    let (s, d) = symbol("=")(s)?;
    let (s, e) = class_new(s)?;
    Ok((
        s,
        BlockingAssignment::HierarchicalVariable(BlockingAssignmentHierarchicalVariable {
            nodes: (a, b, c, d, e),
        }),
    ))
}

#[parser(MaybeRecursive)]
pub fn operator_assignment(s: Span) -> IResult<Span, OperatorAssignment> {
    let (s, a) = variable_lvalue(s)?;
    let (s, b) = assignment_operator(s)?;
    let (s, c) = expression(s)?;
    Ok((s, OperatorAssignment { nodes: (a, b, c) }))
}

#[parser]
pub fn assignment_operator(s: Span) -> IResult<Span, AssignmentOperator> {
    alt((
        map(symbol("="), |x| AssignmentOperator { nodes: (x,) }),
        map(symbol("+="), |x| AssignmentOperator { nodes: (x,) }),
        map(symbol("-="), |x| AssignmentOperator { nodes: (x,) }),
        map(symbol("*="), |x| AssignmentOperator { nodes: (x,) }),
        map(symbol("/="), |x| AssignmentOperator { nodes: (x,) }),
        map(symbol("%="), |x| AssignmentOperator { nodes: (x,) }),
        map(symbol("&="), |x| AssignmentOperator { nodes: (x,) }),
        map(symbol("|="), |x| AssignmentOperator { nodes: (x,) }),
        map(symbol("^="), |x| AssignmentOperator { nodes: (x,) }),
        map(symbol("<<<="), |x| AssignmentOperator { nodes: (x,) }),
        map(symbol(">>>="), |x| AssignmentOperator { nodes: (x,) }),
        map(symbol("<<="), |x| AssignmentOperator { nodes: (x,) }),
        map(symbol(">>="), |x| AssignmentOperator { nodes: (x,) }),
    ))(s)
}

#[parser(MaybeRecursive)]
pub fn nonblocking_assignment(s: Span) -> IResult<Span, NonblockingAssignment> {
    let (s, a) = variable_lvalue(s)?;
    let (s, b) = symbol("<=")(s)?;
    let (s, c) = opt(delay_or_event_control)(s)?;
    let (s, d) = expression(s)?;
    Ok((
        s,
        NonblockingAssignment {
            nodes: (a, b, c, d),
        },
    ))
}

#[parser]
pub fn procedural_continuous_assignment(s: Span) -> IResult<Span, ProceduralContinuousAssignment> {
    alt((
        procedural_continuous_assignment_assign,
        procedural_continuous_assignment_deassign,
        procedural_continuous_assignment_force_variable,
        procedural_continuous_assignment_force_net,
        procedural_continuous_assignment_release_variable,
        procedural_continuous_assignment_release_net,
    ))(s)
}

#[parser]
pub fn procedural_continuous_assignment_assign(
    s: Span,
) -> IResult<Span, ProceduralContinuousAssignment> {
    let (s, a) = keyword("assign")(s)?;
    let (s, b) = variable_assignment(s)?;
    Ok((
        s,
        ProceduralContinuousAssignment::Assign(ProceduralContinuousAssignmentAssign {
            nodes: (a, b),
        }),
    ))
}

#[parser]
pub fn procedural_continuous_assignment_deassign(
    s: Span,
) -> IResult<Span, ProceduralContinuousAssignment> {
    let (s, a) = keyword("deassign")(s)?;
    let (s, b) = variable_lvalue(s)?;
    Ok((
        s,
        ProceduralContinuousAssignment::Deassign(ProceduralContinuousAssignmentDeassign {
            nodes: (a, b),
        }),
    ))
}

#[parser]
pub fn procedural_continuous_assignment_force_variable(
    s: Span,
) -> IResult<Span, ProceduralContinuousAssignment> {
    let (s, a) = keyword("force")(s)?;
    let (s, b) = variable_assignment(s)?;
    Ok((
        s,
        ProceduralContinuousAssignment::ForceVariable(
            ProceduralContinuousAssignmentForceVariable { nodes: (a, b) },
        ),
    ))
}

#[parser]
pub fn procedural_continuous_assignment_force_net(
    s: Span,
) -> IResult<Span, ProceduralContinuousAssignment> {
    let (s, a) = keyword("force")(s)?;
    let (s, b) = net_assignment(s)?;
    Ok((
        s,
        ProceduralContinuousAssignment::ForceNet(ProceduralContinuousAssignmentForceNet {
            nodes: (a, b),
        }),
    ))
}

#[parser]
pub fn procedural_continuous_assignment_release_variable(
    s: Span,
) -> IResult<Span, ProceduralContinuousAssignment> {
    let (s, a) = keyword("release")(s)?;
    let (s, b) = variable_lvalue(s)?;
    Ok((
        s,
        ProceduralContinuousAssignment::ReleaseVariable(
            ProceduralContinuousAssignmentReleaseVariable { nodes: (a, b) },
        ),
    ))
}

#[parser]
pub fn procedural_continuous_assignment_release_net(
    s: Span,
) -> IResult<Span, ProceduralContinuousAssignment> {
    let (s, a) = keyword("release")(s)?;
    let (s, b) = net_lvalue(s)?;
    Ok((
        s,
        ProceduralContinuousAssignment::ReleaseNet(ProceduralContinuousAssignmentReleaseNet {
            nodes: (a, b),
        }),
    ))
}

#[parser(MaybeRecursive)]
pub fn variable_assignment(s: Span) -> IResult<Span, VariableAssignment> {
    let (s, a) = variable_lvalue(s)?;
    let (s, b) = symbol("=")(s)?;
    let (s, c) = expression(s)?;
    Ok((s, VariableAssignment { nodes: (a, b, c) }))
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocking_assignment() {
        parser_test!(
            blocking_assignment,
            "idest = new [3] (isrc)",
            Ok((_, BlockingAssignment::NonrangeVariable(_)))
        );
    }
}