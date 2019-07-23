use crate::ast::*;
use crate::parser::*;
use nom::branch::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

// -----------------------------------------------------------------------------

#[derive(Debug, Node)]
pub struct ConstantFunctionCall {
    pub nodes: (FunctionSubroutineCall,),
}

#[derive(Debug, Node)]
pub struct TfCall {
    pub nodes: (
        PsOrHierarchicalTfIdentifier,
        Vec<AttributeInstance>,
        Option<Paren< ListOfArguments>>,
    ),
}

#[derive(Debug, Node)]
pub enum SystemTfCall {
    ArgOptionl(SystemTfCallArgOptional),
    ArgDataType(SystemTfCallArgDataType),
    ArgExpression(SystemTfCallArgExpression),
}

#[derive(Debug, Node)]
pub struct SystemTfCallArgOptional {
    pub nodes: (
        SystemTfIdentifier,
        Option<Paren< ListOfArguments>>,
    ),
}

#[derive(Debug, Node)]
pub struct SystemTfCallArgDataType {
    pub nodes: (
        SystemTfIdentifier,
        Paren< (DataType, Option<(Symbol, Expression)>)>,
    ),
}

#[derive(Debug, Node)]
pub struct SystemTfCallArgExpression {
    pub nodes: (
        SystemTfIdentifier,
        Paren<
            
            (
                List<Symbol, Option<Expression>>,
                Option<(Symbol, Option<ClockingEvent>)>,
            ),
        >,
    ),
}

#[derive(Debug, Node)]
pub enum SubroutineCall {
    TfCall(Box<TfCall>),
    SystemTfCall(Box<SystemTfCall>),
    MethodCall(Box<MethodCall>),
    Randomize(Box<SubroutineCallRandomize>),
}

#[derive(Debug, Node)]
pub struct SubroutineCallRandomize {
    pub nodes: (Option<(Keyword, Symbol)>, RandomizeCall),
}

#[derive(Debug, Node)]
pub struct FunctionSubroutineCall {
    pub nodes: (SubroutineCall,),
}

#[derive(Debug, Node)]
pub enum ListOfArguments {
    Ordered(ListOfArgumentsOrdered),
    Named(ListOfArgumentsNamed),
}

#[derive(Debug, Node)]
pub struct ListOfArgumentsOrdered {
    pub nodes: (
        List<Symbol, Option<Expression>>,
        Vec<(
            Symbol,
            Symbol,
            Identifier,
            Paren< Option<Expression>>,
        )>,
    ),
}

#[derive(Debug, Node)]
pub struct ListOfArgumentsNamed {
    pub nodes: (
        Symbol,
        Identifier,
        Paren< Option<Expression>>,
        Vec<(
            Symbol,
            Symbol,
            Identifier,
            Paren< Option<Expression>>,
        )>,
    ),
}

#[derive(Debug, Node)]
pub struct MethodCall {
    pub nodes: (MethodCallRoot, Symbol, MethodCallBody),
}

#[derive(Debug, Node)]
pub enum MethodCallBody {
    User(MethodCallBodyUser),
    BuiltInMethodCall(BuiltInMethodCall),
}

#[derive(Debug, Node)]
pub struct MethodCallBodyUser {
    pub nodes: (
        MethodIdentifier,
        Vec<AttributeInstance>,
        Option<Paren< ListOfArguments>>,
    ),
}

#[derive(Debug, Node)]
pub enum BuiltInMethodCall {
    ArrayManipulationCall(ArrayManipulationCall),
    RandomizeCall(RandomizeCall),
}

#[derive(Debug, Node)]
pub struct ArrayManipulationCall {
    pub nodes: (
        ArrayMethodName,
        Vec<AttributeInstance>,
        Option<Paren< ListOfArguments>>,
        Option<(Keyword, Paren< Expression>)>,
    ),
}

#[derive(Debug, Node)]
pub struct RandomizeCall {
    pub nodes: (
        Keyword,
        Vec<AttributeInstance>,
        Option<Paren< Option<VariableIdentifierListOrNull>>>,
        Option<(
            Keyword,
            Option<Paren< Option<IdentifierList>>>,
            ConstraintBlock,
        )>,
    ),
}

#[derive(Debug, Node)]
pub enum VariableIdentifierListOrNull {
    VariableIdentifierList(VariableIdentifierList),
    Null(Keyword),
}

#[derive(Debug, Node)]
pub enum MethodCallRoot {
    Primary(Primary),
    ImplicitClassHandle(ImplicitClassHandle),
}

#[derive(Debug, Node)]
pub enum ArrayMethodName {
    MethodIdentifier(MethodIdentifier),
    Unique(Keyword),
    And(Keyword),
    Or(Keyword),
    Xor(Keyword),
}

// -----------------------------------------------------------------------------

#[parser]
pub fn constant_function_call(s: Span) -> IResult<Span, ConstantFunctionCall> {
    let (s, a) = function_subroutine_call(s)?;
    Ok((s, ConstantFunctionCall { nodes: (a,) }))
}

#[parser]
pub fn tf_call(s: Span) -> IResult<Span, TfCall> {
    let (s, a) = ps_or_hierarchical_tf_identifier(s)?;
    let (s, b) = many0(attribute_instance)(s)?;
    let (s, c) = opt(paren(list_of_arguments))(s)?;
    Ok((s, TfCall { nodes: (a, b, c) }))
}

#[parser]
pub fn system_tf_call(s: Span) -> IResult<Span, SystemTfCall> {
    alt((
        system_tf_call_arg_optional,
        system_tf_call_arg_data_type,
        system_tf_call_arg_expression,
    ))(s)
}

#[parser]
pub fn system_tf_call_arg_optional(s: Span) -> IResult<Span, SystemTfCall> {
    let (s, a) = system_tf_identifier(s)?;
    let (s, b) = opt(paren(list_of_arguments))(s)?;
    Ok((
        s,
        SystemTfCall::ArgOptionl(SystemTfCallArgOptional { nodes: (a, b) }),
    ))
}

#[parser]
pub fn system_tf_call_arg_data_type(s: Span) -> IResult<Span, SystemTfCall> {
    let (s, a) = system_tf_identifier(s)?;
    let (s, b) = paren(pair(data_type, opt(pair(symbol(","), expression))))(s)?;
    Ok((
        s,
        SystemTfCall::ArgDataType(SystemTfCallArgDataType { nodes: (a, b) }),
    ))
}

#[parser]
pub fn system_tf_call_arg_expression(s: Span) -> IResult<Span, SystemTfCall> {
    let (s, a) = system_tf_identifier(s)?;
    let (s, b) = paren(pair(
        list(symbol(","), opt(expression)),
        opt(pair(symbol(","), opt(clocking_event))),
    ))(s)?;
    Ok((
        s,
        SystemTfCall::ArgExpression(SystemTfCallArgExpression { nodes: (a, b) }),
    ))
}

#[parser]
pub fn subroutine_call(s: Span) -> IResult<Span, SubroutineCall> {
    alt((
        map(tf_call, |x| SubroutineCall::TfCall(Box::new(x))),
        map(system_tf_call, |x| {
            SubroutineCall::SystemTfCall(Box::new(x))
        }),
        map(method_call, |x| SubroutineCall::MethodCall(Box::new(x))),
        subroutine_call_randomize,
    ))(s)
}

#[parser]
pub fn subroutine_call_randomize(s: Span) -> IResult<Span, SubroutineCall> {
    let (s, a) = opt(pair(keyword("std"), symbol("::")))(s)?;
    let (s, b) = randomize_call(s)?;
    Ok((
        s,
        SubroutineCall::Randomize(Box::new(SubroutineCallRandomize { nodes: (a, b) })),
    ))
}

#[parser(Memoize)]
pub fn function_subroutine_call(s: Span) -> IResult<Span, FunctionSubroutineCall> {
    map(subroutine_call, |x| FunctionSubroutineCall { nodes: (x,) })(s)
}

#[parser]
pub fn list_of_arguments(s: Span) -> IResult<Span, ListOfArguments> {
    alt((list_of_arguments_ordered, list_of_arguments_named))(s)
}

#[parser(MaybeRecursive)]
pub fn list_of_arguments_ordered(s: Span) -> IResult<Span, ListOfArguments> {
    let (s, a) = list(symbol(","), opt(expression))(s)?;
    let (s, b) = many0(tuple((
        symbol(","),
        symbol("."),
        identifier,
        paren(opt(expression)),
    )))(s)?;
    Ok((
        s,
        ListOfArguments::Ordered(ListOfArgumentsOrdered { nodes: (a, b) }),
    ))
}

#[parser]
pub fn list_of_arguments_named(s: Span) -> IResult<Span, ListOfArguments> {
    let (s, a) = symbol(".")(s)?;
    let (s, b) = identifier(s)?;
    let (s, c) = paren(opt(expression))(s)?;
    let (s, d) = many0(tuple((
        symbol(","),
        symbol("."),
        identifier,
        paren(opt(expression)),
    )))(s)?;
    Ok((
        s,
        ListOfArguments::Named(ListOfArgumentsNamed {
            nodes: (a, b, c, d),
        }),
    ))
}

#[parser(MaybeRecursive)]
pub fn method_call(s: Span) -> IResult<Span, MethodCall> {
    let (s, a) = method_call_root(s)?;
    let (s, b) = symbol(".")(s)?;
    let (s, c) = method_call_body(s)?;

    Ok((s, MethodCall { nodes: (a, b, c) }))
}

#[parser]
pub fn method_call_body(s: Span) -> IResult<Span, MethodCallBody> {
    alt((
        method_call_body_user,
        map(built_in_method_call, |x| {
            MethodCallBody::BuiltInMethodCall(x)
        }),
    ))(s)
}

#[parser]
pub fn method_call_body_user(s: Span) -> IResult<Span, MethodCallBody> {
    let (s, a) = method_identifier(s)?;
    let (s, b) = many0(attribute_instance)(s)?;
    let (s, c) = opt(paren(list_of_arguments))(s)?;
    Ok((
        s,
        MethodCallBody::User(MethodCallBodyUser { nodes: (a, b, c) }),
    ))
}

#[parser]
pub fn built_in_method_call(s: Span) -> IResult<Span, BuiltInMethodCall> {
    alt((
        map(array_manipulation_call, |x| {
            BuiltInMethodCall::ArrayManipulationCall(x)
        }),
        map(randomize_call, |x| BuiltInMethodCall::RandomizeCall(x)),
    ))(s)
}

#[parser]
pub fn array_manipulation_call(s: Span) -> IResult<Span, ArrayManipulationCall> {
    let (s, a) = array_method_name(s)?;
    let (s, b) = many0(attribute_instance)(s)?;
    let (s, c) = opt(paren(list_of_arguments))(s)?;
    let (s, d) = opt(pair(keyword("with"), paren(expression)))(s)?;
    Ok((
        s,
        ArrayManipulationCall {
            nodes: (a, b, c, d),
        },
    ))
}

#[parser]
pub fn randomize_call(s: Span) -> IResult<Span, RandomizeCall> {
    let (s, a) = keyword("randomize")(s)?;
    let (s, b) = many0(attribute_instance)(s)?;
    let (s, c) = opt(paren(opt(variable_identifier_list_or_null)))(s)?;
    let (s, d) = opt(triple(
        keyword("with"),
        opt(paren(opt(identifier_list))),
        constraint_block,
    ))(s)?;
    Ok((
        s,
        RandomizeCall {
            nodes: (a, b, c, d),
        },
    ))
}

#[parser]
pub fn variable_identifier_list_or_null(s: Span) -> IResult<Span, VariableIdentifierListOrNull> {
    alt((
        map(variable_identifier_list, |x| {
            VariableIdentifierListOrNull::VariableIdentifierList(x)
        }),
        map(keyword("null"), |x| VariableIdentifierListOrNull::Null(x)),
    ))(s)
}

#[parser]
pub fn method_call_root(s: Span) -> IResult<Span, MethodCallRoot> {
    alt((
        map(primary, |x| MethodCallRoot::Primary(x)),
        map(implicit_class_handle, |x| {
            MethodCallRoot::ImplicitClassHandle(x)
        }),
    ))(s)
}

#[parser]
pub fn array_method_name(s: Span) -> IResult<Span, ArrayMethodName> {
    alt((
        map(keyword("unique"), |x| ArrayMethodName::Unique(x)),
        map(keyword("and"), |x| ArrayMethodName::And(x)),
        map(keyword("or"), |x| ArrayMethodName::Or(x)),
        map(keyword("xor"), |x| ArrayMethodName::Xor(x)),
        map(method_identifier, |x| ArrayMethodName::MethodIdentifier(x)),
    ))(s)
}

// -----------------------------------------------------------------------------