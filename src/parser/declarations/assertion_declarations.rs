use crate::ast::*;
use crate::parser::*;
use nom::branch::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

// -----------------------------------------------------------------------------

#[derive(Debug, Node)]
pub enum ConcurrentAssertionItem {
    Statement(ConcurrentAssertionItemStatement),
    CheckerInstantiation(CheckerInstantiation),
}

#[derive(Debug, Node)]
pub struct ConcurrentAssertionItemStatement {
    pub nodes: (
        Option<(BlockIdentifier, Symbol)>,
        ConcurrentAssertionStatement,
    ),
}

#[derive(Debug, Node)]
pub enum ConcurrentAssertionStatement {
    AssertPropertyStatement(AssertPropertyStatement),
    AssumePropertyStatement(AssumePropertyStatement),
    CoverPropertyStatement(CoverPropertyStatement),
    CoverSequenceStatement(CoverSequenceStatement),
    RestrictPropertyStatement(RestrictPropertyStatement),
}

#[derive(Debug, Node)]
pub struct AssertPropertyStatement {
    pub nodes: (Keyword, Keyword, Paren<PropertySpec>, ActionBlock),
}

#[derive(Debug, Node)]
pub struct AssumePropertyStatement {
    pub nodes: (Keyword, Keyword, Paren<PropertySpec>, ActionBlock),
}

#[derive(Debug, Node)]
pub struct CoverPropertyStatement {
    pub nodes: (Keyword, Keyword, Paren<PropertySpec>, StatementOrNull),
}

#[derive(Debug, Node)]
pub struct ExpectPropertyStatement {
    pub nodes: (Keyword, Paren<PropertySpec>, ActionBlock),
}

#[derive(Debug, Node)]
pub struct CoverSequenceStatement {
    pub nodes: (
        Keyword,
        Keyword,
        Paren<(
            Option<ClockingEvent>,
            Option<(Keyword, Keyword, Paren<ExpressionOrDist>)>,
            SequenceExpr,
        )>,
        StatementOrNull,
    ),
}

#[derive(Debug, Node)]
pub struct RestrictPropertyStatement {
    pub nodes: (Keyword, Keyword, Paren<PropertySpec>, Symbol),
}

#[derive(Debug, Node)]
pub struct PropertyInstance {
    pub nodes: (
        PsOrHierarchicalPropertyIdentifier,
        Option<Paren<Option<PropertyListOfArguments>>>,
    ),
}

#[derive(Debug, Node)]
pub enum PropertyListOfArguments {
    Ordered(PropertyListOfArgumentsOrdered),
    Named(PropertyListOfArgumentsNamed),
}

#[derive(Debug, Node)]
pub struct PropertyListOfArgumentsOrdered {
    pub nodes: (
        List<Symbol, Option<PropertyActualArg>>,
        Vec<(Symbol, Symbol, Identifier, Paren<Option<PropertyActualArg>>)>,
    ),
}

#[derive(Debug, Node)]
pub struct PropertyListOfArgumentsNamed {
    pub nodes: (List<Symbol, (Symbol, Identifier, Paren<Option<PropertyActualArg>>)>,),
}

#[derive(Debug, Node)]
pub enum PropertyActualArg {
    PropertyExpr(PropertyExpr),
    SequenceActualArg(SequenceActualArg),
}

#[derive(Debug, Node)]
pub enum AssertionItemDeclaration {
    PropertyDeclaration(PropertyDeclaration),
    SequenceDeclaration(SequenceDeclaration),
    LetDeclaration(LetDeclaration),
}

#[derive(Debug, Node)]
pub struct PropertyDeclaration {
    pub nodes: (
        Keyword,
        PropertyIdentifier,
        Option<Paren<Option<PropertyPortList>>>,
        Symbol,
        Vec<AssertionVariableDeclaration>,
        PropertySpec,
        Option<Symbol>,
        Keyword,
        Option<(Symbol, PropertyIdentifier)>,
    ),
}

#[derive(Debug, Node)]
pub struct PropertyPortList {
    pub nodes: (List<Symbol, PropertyPortItem>,),
}

#[derive(Debug, Node)]
pub struct PropertyPortItem {
    pub nodes: (
        Vec<AttributeInstance>,
        Option<(Local, Option<PropertyLvarPortDirection>)>,
        Option<PropertyFormalType>,
        FormalPortIdentifier,
        Vec<VariableDimension>,
        Option<(Symbol, PropertyActualArg)>,
    ),
}

#[derive(Debug, Node)]
pub enum PropertyLvarPortDirection {
    Input(Keyword),
}

#[derive(Debug, Node)]
pub enum PropertyFormalType {
    SequenceFormalType(SequenceFormalType),
    Property(Keyword),
}

#[derive(Debug, Node)]
pub struct PropertySpec {
    pub nodes: (
        Option<ClockingEvent>,
        Option<(Keyword, Keyword, Paren<ExpressionOrDist>)>,
        PropertyExpr,
    ),
}

#[derive(Debug, Node)]
pub enum PropertyExpr {
    SequenceExpr(SequenceExpr),
    Strong(PropertyExprStrong),
    Weak(PropertyExprWeak),
    Paren(Box<PropertyExprParen>),
    Not(Box<PropertyExprNot>),
    Or(Box<PropertyExprOr>),
    And(Box<PropertyExprAnd>),
    ImplicationOverlapped(Box<PropertyExprImplicationOverlapped>),
    ImplicationNonoverlapped(Box<PropertyExprImplicationNonoverlapped>),
    If(Box<PropertyExprIf>),
    Case(Box<PropertyExprCase>),
    FollowedByOverlapped(Box<PropertyExprFollowedByOverlapped>),
    FollowedByNonoverlapped(Box<PropertyExprFollowedByNonoverlapped>),
    Nexttime(Box<PropertyExprNexttime>),
    SNexttime(Box<PropertyExprSNexttime>),
    Always(Box<PropertyExprAlways>),
    SAlways(Box<PropertyExprSAlways>),
    Eventually(Box<PropertyExprEventually>),
    SEventually(Box<PropertyExprSEventually>),
    Until(Box<PropertyExprUntil>),
    SUntil(Box<PropertyExprSUntil>),
    UntilWith(Box<PropertyExprUntilWith>),
    SUntilWith(Box<PropertyExprSUntilWith>),
    Implies(Box<PropertyExprImplies>),
    Iff(Box<PropertyExprIff>),
    AcceptOn(Box<PropertyExprAcceptOn>),
    RejectOn(Box<PropertyExprRejectOn>),
    SyncAcceptOn(Box<PropertyExprSyncAcceptOn>),
    SyncRejectOn(Box<PropertyExprSyncRejectOn>),
    PropertyInstance(Box<PropertyInstance>),
    ClockingEvent(Box<PropertyExprClockingEvent>),
}

#[derive(Debug, Node)]
pub struct PropertyExprStrong {
    pub nodes: (Keyword, Paren<SequenceExpr>),
}

#[derive(Debug, Node)]
pub struct PropertyExprWeak {
    pub nodes: (Keyword, Paren<SequenceExpr>),
}

#[derive(Debug, Node)]
pub struct PropertyExprParen {
    pub nodes: (Paren<SequenceExpr>,),
}

#[derive(Debug, Node)]
pub struct PropertyExprNot {
    pub nodes: (Keyword, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprOr {
    pub nodes: (PropertyExpr, Keyword, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprAnd {
    pub nodes: (PropertyExpr, Keyword, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprImplicationOverlapped {
    pub nodes: (SequenceExpr, Symbol, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprImplicationNonoverlapped {
    pub nodes: (SequenceExpr, Symbol, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprIf {
    pub nodes: (
        Keyword,
        Paren<ExpressionOrDist>,
        PropertyExpr,
        Option<(Keyword, PropertyExpr)>,
    ),
}

#[derive(Debug, Node)]
pub struct PropertyExprCase {
    pub nodes: (
        Keyword,
        Paren<ExpressionOrDist>,
        PropertyCaseItem,
        Vec<PropertyCaseItem>,
        Keyword,
    ),
}

#[derive(Debug, Node)]
pub struct PropertyExprFollowedByOverlapped {
    pub nodes: (SequenceExpr, Symbol, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprFollowedByNonoverlapped {
    pub nodes: (SequenceExpr, Symbol, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprNexttime {
    pub nodes: (Keyword, Option<Bracket<ConstantExpression>>, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprSNexttime {
    pub nodes: (Keyword, Option<Bracket<ConstantExpression>>, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprAlways {
    pub nodes: (
        Keyword,
        Option<Bracket<CycleDelayConstRangeExpression>>,
        PropertyExpr,
    ),
}

#[derive(Debug, Node)]
pub struct PropertyExprSAlways {
    pub nodes: (
        Keyword,
        Bracket<CycleDelayConstRangeExpression>,
        PropertyExpr,
    ),
}

#[derive(Debug, Node)]
pub struct PropertyExprEventually {
    pub nodes: (Keyword, Bracket<ConstantRange>, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprSEventually {
    pub nodes: (
        Keyword,
        Option<Bracket<CycleDelayConstRangeExpression>>,
        PropertyExpr,
    ),
}

#[derive(Debug, Node)]
pub struct PropertyExprUntil {
    pub nodes: (PropertyExpr, Keyword, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprSUntil {
    pub nodes: (PropertyExpr, Keyword, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprUntilWith {
    pub nodes: (PropertyExpr, Keyword, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprSUntilWith {
    pub nodes: (PropertyExpr, Keyword, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprImplies {
    pub nodes: (PropertyExpr, Keyword, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprIff {
    pub nodes: (PropertyExpr, Keyword, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprAcceptOn {
    pub nodes: (Keyword, Paren<ExpressionOrDist>, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprRejectOn {
    pub nodes: (Keyword, Paren<ExpressionOrDist>, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprSyncAcceptOn {
    pub nodes: (Keyword, Paren<ExpressionOrDist>, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprSyncRejectOn {
    pub nodes: (Keyword, Paren<ExpressionOrDist>, PropertyExpr),
}

#[derive(Debug, Node)]
pub struct PropertyExprClockingEvent {
    pub nodes: (ClockingEvent, PropertyExpr),
}

#[derive(Debug, Node)]
pub enum PropertyCaseItem {
    Nondefault(PropertyCaseItemNondefault),
    Default(PropertyCaseItemDefault),
}

#[derive(Debug, Node)]
pub struct PropertyCaseItemNondefault {
    pub nodes: (List<Symbol, ExpressionOrDist>, Symbol, PropertyExpr, Symbol),
}

#[derive(Debug, Node)]
pub struct PropertyCaseItemDefault {
    pub nodes: (Keyword, Option<Symbol>, PropertyExpr, Symbol),
}

#[derive(Debug, Node)]
pub struct SequenceDeclaration {
    pub nodes: (
        Keyword,
        SequenceIdentifier,
        Option<Paren<Option<SequencePortList>>>,
        Symbol,
        Vec<AssertionVariableDeclaration>,
        SequenceExpr,
        Option<Symbol>,
        Keyword,
        Option<(Symbol, SequenceIdentifier)>,
    ),
}

#[derive(Debug, Node)]
pub struct SequencePortList {
    pub nodes: (List<Symbol, SequencePortItem>,),
}

#[derive(Debug, Node)]
pub struct SequencePortItem {
    pub nodes: (
        Vec<AttributeInstance>,
        Option<(Local, Option<SequenceLvarPortDirection>)>,
        Option<SequenceFormalType>,
        FormalPortIdentifier,
        Vec<VariableDimension>,
        Option<(Symbol, SequenceActualArg)>,
    ),
}

#[derive(Debug, Node)]
pub enum SequenceLvarPortDirection {
    Input(Keyword),
    Inout(Keyword),
    Output(Keyword),
}

#[derive(Debug, Node)]
pub enum SequenceFormalType {
    DataTypeOrImplicit(DataTypeOrImplicit),
    Sequence(Keyword),
    Untyped(Keyword),
}

#[derive(Debug, Node)]
pub enum SequenceExpr {
    CycleDelayExpr(Box<SequenceExprCycleDelayExpr>),
    ExprCycleDelayExpr(Box<SequenceExprExprCycleDelayExpr>),
    Expression(SequenceExprExpression),
    Instance(Box<SequenceExprInstance>),
    Paren(Box<SequenceExprParen>),
    And(Box<SequenceExprAnd>),
    Intersect(Box<SequenceExprIntersect>),
    Or(Box<SequenceExprOr>),
    FirstMatch(Box<SequenceExprFirstMatch>),
    Throughout(Box<SequenceExprThroughout>),
    Within(Box<SequenceExprWithin>),
    ClockingEvent(Box<SequenceExprClockingEvent>),
}

#[derive(Debug, Node)]
pub struct SequenceExprCycleDelayExpr {
    pub nodes: (
        CycleDelayRange,
        SequenceExpr,
        Vec<(CycleDelayRange, SequenceExpr)>,
    ),
}

#[derive(Debug, Node)]
pub struct SequenceExprExprCycleDelayExpr {
    pub nodes: (
        SequenceExpr,
        CycleDelayRange,
        SequenceExpr,
        Vec<(CycleDelayRange, SequenceExpr)>,
    ),
}

#[derive(Debug, Node)]
pub struct SequenceExprExpression {
    pub nodes: (ExpressionOrDist, Option<BooleanAbbrev>),
}

#[derive(Debug, Node)]
pub struct SequenceExprInstance {
    pub nodes: (SequenceInstance, Option<SequenceAbbrev>),
}

#[derive(Debug, Node)]
pub struct SequenceExprParen {
    pub nodes: (
        Paren<(SequenceExpr, Vec<(Symbol, SequenceMatchItem)>)>,
        Option<SequenceAbbrev>,
    ),
}

#[derive(Debug, Node)]
pub struct SequenceExprAnd {
    pub nodes: (SequenceExpr, Keyword, SequenceExpr),
}

#[derive(Debug, Node)]
pub struct SequenceExprIntersect {
    pub nodes: (SequenceExpr, Keyword, SequenceExpr),
}

#[derive(Debug, Node)]
pub struct SequenceExprOr {
    pub nodes: (SequenceExpr, Keyword, SequenceExpr),
}

#[derive(Debug, Node)]
pub struct SequenceExprFirstMatch {
    pub nodes: (
        Keyword,
        Paren<(SequenceExpr, Vec<(Symbol, SequenceMatchItem)>)>,
    ),
}

#[derive(Debug, Node)]
pub struct SequenceExprThroughout {
    pub nodes: (ExpressionOrDist, Keyword, SequenceExpr),
}

#[derive(Debug, Node)]
pub struct SequenceExprWithin {
    pub nodes: (SequenceExpr, Keyword, SequenceExpr),
}

#[derive(Debug, Node)]
pub struct SequenceExprClockingEvent {
    pub nodes: (ClockingEvent, SequenceExpr),
}

#[derive(Debug, Node)]
pub enum CycleDelayRange {
    Primary(CycleDelayRangePrimary),
    Expression(CycleDelayRangeExpression),
    Asterisk(CycleDelayRangeAsterisk),
    Plus(CycleDelayRangePlus),
}

#[derive(Debug, Node)]
pub struct CycleDelayRangePrimary {
    pub nodes: (Symbol, ConstantPrimary),
}

#[derive(Debug, Node)]
pub struct CycleDelayRangeExpression {
    pub nodes: (Symbol, Bracket<CycleDelayConstRangeExpression>),
}

#[derive(Debug, Node)]
pub struct CycleDelayRangeAsterisk {
    pub nodes: (Symbol, Bracket<Symbol>),
}

#[derive(Debug, Node)]
pub struct CycleDelayRangePlus {
    pub nodes: (Symbol, Bracket<Symbol>),
}

#[derive(Debug, Node)]
pub struct SequenceMethodCall {
    pub nodes: (SequenceInstance, Symbol, MethodIdentifier),
}

#[derive(Debug, Node)]
pub enum SequenceMatchItem {
    OperatorAssignment(OperatorAssignment),
    IncOrDecExpression(IncOrDecExpression),
    SubroutineCall(SubroutineCall),
}

#[derive(Debug, Node)]
pub struct SequenceInstance {
    pub nodes: (
        PsOrHierarchicalSequenceIdentifier,
        Option<Paren<Option<SequenceListOfArguments>>>,
    ),
}

#[derive(Debug, Node)]
pub enum SequenceListOfArguments {
    Ordered(SequenceListOfArgumentsOrdered),
    Named(SequenceListOfArgumentsNamed),
}

#[derive(Debug, Node)]
pub struct SequenceListOfArgumentsOrdered {
    pub nodes: (
        List<Symbol, Option<SequenceActualArg>>,
        Vec<(Symbol, Symbol, Identifier, Paren<Option<SequenceActualArg>>)>,
    ),
}

#[derive(Debug, Node)]
pub struct SequenceListOfArgumentsNamed {
    pub nodes: (List<Symbol, (Symbol, Identifier, Paren<Option<SequenceActualArg>>)>,),
}

#[derive(Debug, Node)]
pub enum SequenceActualArg {
    EventExpression(EventExpression),
    SequenceExpr(SequenceExpr),
}

#[derive(Debug, Node)]
pub enum BooleanAbbrev {
    ConsecutiveRepetition(ConsecutiveRepetition),
    NonConsecutiveRepetition(NonConsecutiveRepetition),
    GotoRepetition(GotoRepetition),
}

#[derive(Debug, Node)]
pub struct SequenceAbbrev {
    pub nodes: (ConsecutiveRepetition,),
}

#[derive(Debug, Node)]
pub enum ConsecutiveRepetition {
    Expression(ConsecutiveRepetitionExpression),
    Asterisk(ConsecutiveRepetitionAsterisk),
    Plus(ConsecutiveRepetitionPlus),
}

#[derive(Debug, Node)]
pub struct ConsecutiveRepetitionExpression {
    pub nodes: (Bracket<(Symbol, ConstOrRangeExpression)>,),
}

#[derive(Debug, Node)]
pub struct ConsecutiveRepetitionAsterisk {
    pub nodes: (Bracket<Symbol>,),
}

#[derive(Debug, Node)]
pub struct ConsecutiveRepetitionPlus {
    pub nodes: (Bracket<Symbol>,),
}

#[derive(Debug, Node)]
pub struct NonConsecutiveRepetition {
    pub nodes: (Bracket<(Symbol, ConstOrRangeExpression)>,),
}

#[derive(Debug, Node)]
pub struct GotoRepetition {
    pub nodes: (Bracket<(Symbol, ConstOrRangeExpression)>,),
}

#[derive(Debug, Node)]
pub enum ConstOrRangeExpression {
    ConstantExpression(ConstantExpression),
    CycleDelayConstRangeExpression(CycleDelayConstRangeExpression),
}

#[derive(Debug, Node)]
pub enum CycleDelayConstRangeExpression {
    Binary(CycleDelayConstRangeExpressionBinary),
    Dollar(CycleDelayConstRangeExpressionDollar),
}

#[derive(Debug, Node)]
pub struct CycleDelayConstRangeExpressionBinary {
    pub nodes: (ConstantExpression, Symbol, ConstantExpression),
}

#[derive(Debug, Node)]
pub struct CycleDelayConstRangeExpressionDollar {
    pub nodes: (ConstantExpression, Symbol, Symbol),
}

#[derive(Debug, Node)]
pub struct ExpressionOrDist {
    pub nodes: (Expression, Option<(Keyword, Brace<DistList>)>),
}

#[derive(Debug, Node)]
pub struct AssertionVariableDeclaration {
    pub nodes: (VarDataType, ListOfVariableDeclAssignments, Symbol),
}

// -----------------------------------------------------------------------------

#[parser]
pub fn concurrent_assertion_item(s: Span) -> IResult<Span, ConcurrentAssertionItem> {
    alt((
        concurrent_assertion_item_statement,
        map(checker_instantiation, |x| {
            ConcurrentAssertionItem::CheckerInstantiation(x)
        }),
    ))(s)
}

#[parser]
pub fn concurrent_assertion_item_statement(s: Span) -> IResult<Span, ConcurrentAssertionItem> {
    let (s, a) = opt(pair(block_identifier, symbol(":")))(s)?;
    let (s, b) = concurrent_assertion_statement(s)?;
    Ok((
        s,
        ConcurrentAssertionItem::Statement(ConcurrentAssertionItemStatement { nodes: (a, b) }),
    ))
}

#[parser]
pub fn concurrent_assertion_statement(s: Span) -> IResult<Span, ConcurrentAssertionStatement> {
    alt((
        map(assert_property_statement, |x| {
            ConcurrentAssertionStatement::AssertPropertyStatement(x)
        }),
        map(assume_property_statement, |x| {
            ConcurrentAssertionStatement::AssumePropertyStatement(x)
        }),
        map(cover_property_statement, |x| {
            ConcurrentAssertionStatement::CoverPropertyStatement(x)
        }),
        map(cover_sequence_statement, |x| {
            ConcurrentAssertionStatement::CoverSequenceStatement(x)
        }),
        map(restrict_property_statement, |x| {
            ConcurrentAssertionStatement::RestrictPropertyStatement(x)
        }),
    ))(s)
}

#[parser]
pub fn assert_property_statement(s: Span) -> IResult<Span, AssertPropertyStatement> {
    let (s, a) = keyword("assert")(s)?;
    let (s, b) = keyword("property")(s)?;
    let (s, c) = paren(property_spec)(s)?;
    let (s, d) = action_block(s)?;
    Ok((
        s,
        AssertPropertyStatement {
            nodes: (a, b, c, d),
        },
    ))
}

#[parser]
pub fn assume_property_statement(s: Span) -> IResult<Span, AssumePropertyStatement> {
    let (s, a) = keyword("assume")(s)?;
    let (s, b) = keyword("property")(s)?;
    let (s, c) = paren(property_spec)(s)?;
    let (s, d) = action_block(s)?;
    Ok((
        s,
        AssumePropertyStatement {
            nodes: (a, b, c, d),
        },
    ))
}

#[parser]
pub fn cover_property_statement(s: Span) -> IResult<Span, CoverPropertyStatement> {
    let (s, a) = keyword("cover")(s)?;
    let (s, b) = keyword("property")(s)?;
    let (s, c) = paren(property_spec)(s)?;
    let (s, d) = statement_or_null(s)?;
    Ok((
        s,
        CoverPropertyStatement {
            nodes: (a, b, c, d),
        },
    ))
}

#[parser]
pub fn expect_property_statement(s: Span) -> IResult<Span, ExpectPropertyStatement> {
    let (s, a) = keyword("expect")(s)?;
    let (s, b) = paren(property_spec)(s)?;
    let (s, c) = action_block(s)?;
    Ok((s, ExpectPropertyStatement { nodes: (a, b, c) }))
}

#[parser]
pub fn cover_sequence_statement(s: Span) -> IResult<Span, CoverSequenceStatement> {
    let (s, a) = keyword("cover")(s)?;
    let (s, b) = keyword("sequence")(s)?;
    let (s, c) = paren(triple(
        opt(clocking_event),
        opt(triple(
            keyword("disable"),
            keyword("iff"),
            paren(expression_or_dist),
        )),
        sequence_expr,
    ))(s)?;
    let (s, d) = statement_or_null(s)?;
    Ok((
        s,
        CoverSequenceStatement {
            nodes: (a, b, c, d),
        },
    ))
}

#[parser]
pub fn restrict_property_statement(s: Span) -> IResult<Span, RestrictPropertyStatement> {
    let (s, a) = keyword("restrict")(s)?;
    let (s, b) = keyword("property")(s)?;
    let (s, c) = paren(property_spec)(s)?;
    let (s, d) = symbol(";")(s)?;
    Ok((
        s,
        RestrictPropertyStatement {
            nodes: (a, b, c, d),
        },
    ))
}

#[parser]
pub fn property_instance(s: Span) -> IResult<Span, PropertyInstance> {
    let (s, a) = ps_or_hierarchical_property_identifier(s)?;
    let (s, b) = opt(paren(opt(property_list_of_arguments)))(s)?;
    Ok((s, PropertyInstance { nodes: (a, b) }))
}

#[parser]
pub fn property_list_of_arguments(s: Span) -> IResult<Span, PropertyListOfArguments> {
    alt((
        property_list_of_arguments_ordered,
        property_list_of_arguments_named,
    ))(s)
}

#[parser(MaybeRecursive)]
pub fn property_list_of_arguments_ordered(s: Span) -> IResult<Span, PropertyListOfArguments> {
    let (s, a) = list(symbol(","), opt(property_actual_arg))(s)?;
    let (s, b) = many0(tuple((
        symbol(","),
        symbol("."),
        identifier,
        paren(opt(property_actual_arg)),
    )))(s)?;
    Ok((
        s,
        PropertyListOfArguments::Ordered(PropertyListOfArgumentsOrdered { nodes: (a, b) }),
    ))
}

#[parser]
pub fn property_list_of_arguments_named(s: Span) -> IResult<Span, PropertyListOfArguments> {
    let (s, a) = list(
        symbol(","),
        triple(symbol("."), identifier, paren(opt(property_actual_arg))),
    )(s)?;
    Ok((
        s,
        PropertyListOfArguments::Named(PropertyListOfArgumentsNamed { nodes: (a,) }),
    ))
}

#[parser]
pub fn property_actual_arg(s: Span) -> IResult<Span, PropertyActualArg> {
    alt((
        map(property_expr, |x| PropertyActualArg::PropertyExpr(x)),
        map(sequence_actual_arg, |x| {
            PropertyActualArg::SequenceActualArg(x)
        }),
    ))(s)
}

#[parser]
pub fn assertion_item_declaration(s: Span) -> IResult<Span, AssertionItemDeclaration> {
    alt((
        map(property_declaration, |x| {
            AssertionItemDeclaration::PropertyDeclaration(x)
        }),
        map(sequence_declaration, |x| {
            AssertionItemDeclaration::SequenceDeclaration(x)
        }),
        map(let_declaration, |x| {
            AssertionItemDeclaration::LetDeclaration(x)
        }),
    ))(s)
}

#[parser]
pub fn property_declaration(s: Span) -> IResult<Span, PropertyDeclaration> {
    let (s, a) = keyword("property")(s)?;
    let (s, b) = property_identifier(s)?;
    let (s, c) = opt(paren(opt(property_port_list)))(s)?;
    let (s, d) = symbol(";")(s)?;
    let (s, e) = many0(assertion_variable_declaration)(s)?;
    let (s, f) = property_spec(s)?;
    let (s, g) = opt(symbol(";"))(s)?;
    let (s, h) = keyword("endproperty")(s)?;
    let (s, i) = opt(pair(symbol(":"), property_identifier))(s)?;
    Ok((
        s,
        PropertyDeclaration {
            nodes: (a, b, c, d, e, f, g, h, i),
        },
    ))
}

#[parser]
pub fn property_port_list(s: Span) -> IResult<Span, PropertyPortList> {
    let (s, a) = list(symbol(","), property_port_item)(s)?;
    Ok((s, PropertyPortList { nodes: (a,) }))
}

#[parser(Ambiguous)]
pub fn property_port_item(s: Span) -> IResult<Span, PropertyPortItem> {
    let (s, a) = many0(attribute_instance)(s)?;
    let (s, b) = opt(pair(local, opt(property_lvar_port_direction)))(s)?;
    let (s, c) = ambiguous_opt(property_formal_type)(s)?;
    let (s, d) = formal_port_identifier(s)?;
    let (s, e) = many0(variable_dimension)(s)?;
    let (s, f) = opt(pair(symbol("="), property_actual_arg))(s)?;
    Ok((
        s,
        PropertyPortItem {
            nodes: (a, b, c, d, e, f),
        },
    ))
}

#[parser]
pub fn property_lvar_port_direction(s: Span) -> IResult<Span, PropertyLvarPortDirection> {
    let (s, a) = keyword("input")(s)?;
    Ok((s, PropertyLvarPortDirection::Input(a)))
}

#[parser]
pub fn property_formal_type(s: Span) -> IResult<Span, PropertyFormalType> {
    alt((
        map(sequence_formal_type, |x| {
            PropertyFormalType::SequenceFormalType(x)
        }),
        map(keyword("property"), |x| PropertyFormalType::Property(x)),
    ))(s)
}

#[parser(MaybeRecursive)]
pub fn property_spec(s: Span) -> IResult<Span, PropertySpec> {
    let (s, a) = opt(clocking_event)(s)?;
    let (s, b) = opt(triple(
        keyword("disable"),
        keyword("iff"),
        paren(expression_or_dist),
    ))(s)?;
    let (s, c) = property_expr(s)?;
    Ok((s, PropertySpec { nodes: (a, b, c) }))
}

#[parser]
pub fn property_expr(s: Span) -> IResult<Span, PropertyExpr> {
    alt((
        alt((
            map(sequence_expr, |x| PropertyExpr::SequenceExpr(x)),
            property_expr_strong,
            property_expr_weak,
            property_expr_paren,
            property_expr_not,
            property_expr_or,
            property_expr_and,
            property_expr_implication_overlapped,
            property_expr_implication_nonoverlapped,
            property_expr_if,
            property_expr_case,
            property_expr_followed_by_overlapped,
            property_expr_followed_by_nonoverlapped,
            property_expr_nexttime,
            property_expr_s_nexttime,
        )),
        alt((
            property_expr_always,
            property_expr_s_always,
            property_expr_eventually,
            property_expr_s_eventually,
            property_expr_until,
            property_expr_s_until,
            property_expr_until_with,
            property_expr_s_until_with,
            property_expr_implies,
            property_expr_iff,
            property_expr_accept_on,
            property_expr_reject_on,
            property_expr_sync_accept_on,
            property_expr_sync_reject_on,
            map(property_instance, |x| {
                PropertyExpr::PropertyInstance(Box::new(x))
            }),
            property_expr_clocking_event,
        )),
    ))(s)
}

#[parser]
pub fn property_expr_strong(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = keyword("strong")(s)?;
    let (s, b) = paren(sequence_expr)(s)?;
    Ok((
        s,
        PropertyExpr::Strong(PropertyExprStrong { nodes: (a, b) }),
    ))
}

#[parser]
pub fn property_expr_weak(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = keyword("weak")(s)?;
    let (s, b) = paren(sequence_expr)(s)?;
    Ok((
        s,
        PropertyExpr::Strong(PropertyExprStrong { nodes: (a, b) }),
    ))
}

#[parser]
pub fn property_expr_paren(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = paren(sequence_expr)(s)?;
    Ok((
        s,
        PropertyExpr::Paren(Box::new(PropertyExprParen { nodes: (a,) })),
    ))
}

#[parser]
pub fn property_expr_not(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = keyword("not")(s)?;
    let (s, b) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::Not(Box::new(PropertyExprNot { nodes: (a, b) })),
    ))
}

#[parser(MaybeRecursive)]
pub fn property_expr_or(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = property_expr(s)?;
    let (s, b) = keyword("or")(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::Or(Box::new(PropertyExprOr { nodes: (a, b, c) })),
    ))
}

#[parser(MaybeRecursive)]
pub fn property_expr_and(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = property_expr(s)?;
    let (s, b) = keyword("and")(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::And(Box::new(PropertyExprAnd { nodes: (a, b, c) })),
    ))
}

#[parser(MaybeRecursive)]
pub fn property_expr_implication_overlapped(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = sequence_expr(s)?;
    let (s, b) = symbol("|->")(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::ImplicationOverlapped(Box::new(PropertyExprImplicationOverlapped {
            nodes: (a, b, c),
        })),
    ))
}

#[parser(MaybeRecursive)]
pub fn property_expr_implication_nonoverlapped(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = sequence_expr(s)?;
    let (s, b) = symbol("|=>")(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::ImplicationNonoverlapped(Box::new(PropertyExprImplicationNonoverlapped {
            nodes: (a, b, c),
        })),
    ))
}

#[parser]
pub fn property_expr_if(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = keyword("if")(s)?;
    let (s, b) = paren(expression_or_dist)(s)?;
    let (s, c) = property_expr(s)?;
    let (s, d) = opt(pair(keyword("else"), property_expr))(s)?;
    Ok((
        s,
        PropertyExpr::If(Box::new(PropertyExprIf {
            nodes: (a, b, c, d),
        })),
    ))
}

#[parser]
pub fn property_expr_case(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = keyword("case")(s)?;
    let (s, b) = paren(expression_or_dist)(s)?;
    let (s, c) = property_case_item(s)?;
    let (s, d) = many0(property_case_item)(s)?;
    let (s, e) = keyword("endcase")(s)?;
    Ok((
        s,
        PropertyExpr::Case(Box::new(PropertyExprCase {
            nodes: (a, b, c, d, e),
        })),
    ))
}

#[parser(MaybeRecursive)]
pub fn property_expr_followed_by_overlapped(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = sequence_expr(s)?;
    let (s, b) = symbol("#-#")(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::FollowedByOverlapped(Box::new(PropertyExprFollowedByOverlapped {
            nodes: (a, b, c),
        })),
    ))
}

#[parser(MaybeRecursive)]
pub fn property_expr_followed_by_nonoverlapped(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = sequence_expr(s)?;
    let (s, b) = symbol("#=#")(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::FollowedByNonoverlapped(Box::new(PropertyExprFollowedByNonoverlapped {
            nodes: (a, b, c),
        })),
    ))
}

#[parser]
pub fn property_expr_nexttime(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = keyword("nexttime")(s)?;
    let (s, b) = opt(bracket(constant_expression))(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::Nexttime(Box::new(PropertyExprNexttime { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn property_expr_s_nexttime(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = keyword("s_nexttime")(s)?;
    let (s, b) = opt(bracket(constant_expression))(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::SNexttime(Box::new(PropertyExprSNexttime { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn property_expr_always(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = keyword("always")(s)?;
    let (s, b) = opt(bracket(cycle_delay_const_range_expression))(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::Always(Box::new(PropertyExprAlways { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn property_expr_s_always(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = keyword("s_always")(s)?;
    let (s, b) = bracket(cycle_delay_const_range_expression)(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::SAlways(Box::new(PropertyExprSAlways { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn property_expr_eventually(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = keyword("eventually")(s)?;
    let (s, b) = bracket(constant_range)(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::Eventually(Box::new(PropertyExprEventually { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn property_expr_s_eventually(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = keyword("s_eventually")(s)?;
    let (s, b) = opt(bracket(cycle_delay_const_range_expression))(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::SEventually(Box::new(PropertyExprSEventually { nodes: (a, b, c) })),
    ))
}

#[parser(MaybeRecursive)]
pub fn property_expr_until(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = property_expr(s)?;
    let (s, b) = keyword("until")(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::Until(Box::new(PropertyExprUntil { nodes: (a, b, c) })),
    ))
}

#[parser(MaybeRecursive)]
pub fn property_expr_s_until(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = property_expr(s)?;
    let (s, b) = keyword("s_until")(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::SUntil(Box::new(PropertyExprSUntil { nodes: (a, b, c) })),
    ))
}

#[parser(MaybeRecursive)]
pub fn property_expr_until_with(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = property_expr(s)?;
    let (s, b) = keyword("until_with")(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::UntilWith(Box::new(PropertyExprUntilWith { nodes: (a, b, c) })),
    ))
}

#[parser(MaybeRecursive)]
pub fn property_expr_s_until_with(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = property_expr(s)?;
    let (s, b) = keyword("s_until_with")(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::SUntilWith(Box::new(PropertyExprSUntilWith { nodes: (a, b, c) })),
    ))
}

#[parser(MaybeRecursive)]
pub fn property_expr_implies(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = property_expr(s)?;
    let (s, b) = keyword("implies")(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::Implies(Box::new(PropertyExprImplies { nodes: (a, b, c) })),
    ))
}

#[parser(MaybeRecursive)]
pub fn property_expr_iff(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = property_expr(s)?;
    let (s, b) = keyword("iff")(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::Iff(Box::new(PropertyExprIff { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn property_expr_accept_on(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = keyword("accept_on")(s)?;
    let (s, b) = paren(expression_or_dist)(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::AcceptOn(Box::new(PropertyExprAcceptOn { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn property_expr_reject_on(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = keyword("reject_on")(s)?;
    let (s, b) = paren(expression_or_dist)(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::RejectOn(Box::new(PropertyExprRejectOn { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn property_expr_sync_accept_on(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = keyword("sync_accept_on")(s)?;
    let (s, b) = paren(expression_or_dist)(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::SyncAcceptOn(Box::new(PropertyExprSyncAcceptOn { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn property_expr_sync_reject_on(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = keyword("sync_reject_on")(s)?;
    let (s, b) = paren(expression_or_dist)(s)?;
    let (s, c) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::SyncRejectOn(Box::new(PropertyExprSyncRejectOn { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn property_expr_clocking_event(s: Span) -> IResult<Span, PropertyExpr> {
    let (s, a) = clocking_event(s)?;
    let (s, b) = property_expr(s)?;
    Ok((
        s,
        PropertyExpr::ClockingEvent(Box::new(PropertyExprClockingEvent { nodes: (a, b) })),
    ))
}

#[parser]
pub fn property_case_item(s: Span) -> IResult<Span, PropertyCaseItem> {
    alt((property_case_item_nondefault, property_case_item_default))(s)
}

#[parser(MaybeRecursive)]
pub fn property_case_item_nondefault(s: Span) -> IResult<Span, PropertyCaseItem> {
    let (s, a) = list(symbol(","), expression_or_dist)(s)?;
    let (s, b) = symbol(":")(s)?;
    let (s, c) = property_expr(s)?;
    let (s, d) = symbol(";")(s)?;
    Ok((
        s,
        PropertyCaseItem::Nondefault(PropertyCaseItemNondefault {
            nodes: (a, b, c, d),
        }),
    ))
}

#[parser]
pub fn property_case_item_default(s: Span) -> IResult<Span, PropertyCaseItem> {
    let (s, a) = keyword("default")(s)?;
    let (s, b) = opt(symbol(":"))(s)?;
    let (s, c) = property_expr(s)?;
    let (s, d) = symbol(";")(s)?;
    Ok((
        s,
        PropertyCaseItem::Default(PropertyCaseItemDefault {
            nodes: (a, b, c, d),
        }),
    ))
}

#[parser]
pub fn sequence_declaration(s: Span) -> IResult<Span, SequenceDeclaration> {
    let (s, a) = keyword("sequence")(s)?;
    let (s, b) = sequence_identifier(s)?;
    let (s, c) = opt(paren(opt(sequence_port_list)))(s)?;
    let (s, d) = symbol(";")(s)?;
    let (s, e) = many0(assertion_variable_declaration)(s)?;
    let (s, f) = sequence_expr(s)?;
    let (s, g) = opt(symbol(";"))(s)?;
    let (s, h) = keyword("endsequence")(s)?;
    let (s, i) = opt(pair(symbol(":"), sequence_identifier))(s)?;
    Ok((
        s,
        SequenceDeclaration {
            nodes: (a, b, c, d, e, f, g, h, i),
        },
    ))
}

#[parser]
pub fn sequence_port_list(s: Span) -> IResult<Span, SequencePortList> {
    let (s, a) = list(symbol(","), sequence_port_item)(s)?;
    Ok((s, SequencePortList { nodes: (a,) }))
}

#[parser(Ambiguous)]
pub fn sequence_port_item(s: Span) -> IResult<Span, SequencePortItem> {
    let (s, a) = many0(attribute_instance)(s)?;
    let (s, b) = opt(pair(local, opt(sequence_lvar_port_direction)))(s)?;
    let (s, c) = ambiguous_opt(sequence_formal_type)(s)?;
    let (s, d) = formal_port_identifier(s)?;
    let (s, e) = many0(variable_dimension)(s)?;
    let (s, f) = opt(pair(symbol("="), sequence_actual_arg))(s)?;
    Ok((
        s,
        SequencePortItem {
            nodes: (a, b, c, d, e, f),
        },
    ))
}

#[parser]
pub fn sequence_lvar_port_direction(s: Span) -> IResult<Span, SequenceLvarPortDirection> {
    alt((
        map(keyword("input"), |x| SequenceLvarPortDirection::Input(x)),
        map(keyword("inout"), |x| SequenceLvarPortDirection::Inout(x)),
        map(keyword("output"), |x| SequenceLvarPortDirection::Output(x)),
    ))(s)
}

#[parser]
pub fn sequence_formal_type(s: Span) -> IResult<Span, SequenceFormalType> {
    alt((
        map(data_type_or_implicit, |x| {
            SequenceFormalType::DataTypeOrImplicit(x)
        }),
        map(keyword("sequence"), |x| SequenceFormalType::Sequence(x)),
        map(keyword("untyped"), |x| SequenceFormalType::Untyped(x)),
    ))(s)
}

#[parser]
pub fn sequence_expr(s: Span) -> IResult<Span, SequenceExpr> {
    alt((
        sequence_expr_cycle_delay_expr,
        sequence_expr_expr_cycle_delay_expr,
        sequence_expr_expression,
        sequence_expr_instance,
        sequence_expr_paren,
        sequence_expr_and,
        sequence_expr_intersect,
        sequence_expr_or,
        sequence_expr_first_match,
        sequence_expr_throughout,
        sequence_expr_within,
        sequence_expr_clocking_event,
    ))(s)
}

#[parser]
pub fn sequence_expr_cycle_delay_expr(s: Span) -> IResult<Span, SequenceExpr> {
    let (s, a) = cycle_delay_range(s)?;
    let (s, b) = sequence_expr(s)?;
    let (s, c) = many0(pair(cycle_delay_range, sequence_expr))(s)?;
    Ok((
        s,
        SequenceExpr::CycleDelayExpr(Box::new(SequenceExprCycleDelayExpr { nodes: (a, b, c) })),
    ))
}

#[parser(MaybeRecursive)]
pub fn sequence_expr_expr_cycle_delay_expr(s: Span) -> IResult<Span, SequenceExpr> {
    let (s, a) = sequence_expr(s)?;
    let (s, b) = cycle_delay_range(s)?;
    let (s, c) = sequence_expr(s)?;
    let (s, d) = many0(pair(cycle_delay_range, sequence_expr))(s)?;
    Ok((
        s,
        SequenceExpr::ExprCycleDelayExpr(Box::new(SequenceExprExprCycleDelayExpr {
            nodes: (a, b, c, d),
        })),
    ))
}

#[parser(MaybeRecursive)]
pub fn sequence_expr_expression(s: Span) -> IResult<Span, SequenceExpr> {
    let (s, a) = expression_or_dist(s)?;
    let (s, b) = opt(boolean_abbrev)(s)?;
    Ok((
        s,
        SequenceExpr::Expression(SequenceExprExpression { nodes: (a, b) }),
    ))
}

#[parser]
pub fn sequence_expr_instance(s: Span) -> IResult<Span, SequenceExpr> {
    let (s, a) = sequence_instance(s)?;
    let (s, b) = opt(sequence_abbrev)(s)?;
    Ok((
        s,
        SequenceExpr::Instance(Box::new(SequenceExprInstance { nodes: (a, b) })),
    ))
}

#[parser]
pub fn sequence_expr_paren(s: Span) -> IResult<Span, SequenceExpr> {
    let (s, a) = paren(pair(
        sequence_expr,
        many0(pair(symbol(","), sequence_match_item)),
    ))(s)?;
    let (s, b) = opt(sequence_abbrev)(s)?;
    Ok((
        s,
        SequenceExpr::Paren(Box::new(SequenceExprParen { nodes: (a, b) })),
    ))
}

#[parser(MaybeRecursive)]
pub fn sequence_expr_and(s: Span) -> IResult<Span, SequenceExpr> {
    let (s, a) = sequence_expr(s)?;
    let (s, b) = keyword("and")(s)?;
    let (s, c) = sequence_expr(s)?;
    Ok((
        s,
        SequenceExpr::And(Box::new(SequenceExprAnd { nodes: (a, b, c) })),
    ))
}

#[parser(MaybeRecursive)]
pub fn sequence_expr_intersect(s: Span) -> IResult<Span, SequenceExpr> {
    let (s, a) = sequence_expr(s)?;
    let (s, b) = keyword("intersect")(s)?;
    let (s, c) = sequence_expr(s)?;
    Ok((
        s,
        SequenceExpr::Intersect(Box::new(SequenceExprIntersect { nodes: (a, b, c) })),
    ))
}

#[parser(MaybeRecursive)]
pub fn sequence_expr_or(s: Span) -> IResult<Span, SequenceExpr> {
    let (s, a) = sequence_expr(s)?;
    let (s, b) = keyword("or")(s)?;
    let (s, c) = sequence_expr(s)?;
    Ok((
        s,
        SequenceExpr::Or(Box::new(SequenceExprOr { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn sequence_expr_first_match(s: Span) -> IResult<Span, SequenceExpr> {
    let (s, a) = keyword("first_match")(s)?;
    let (s, b) = paren(pair(
        sequence_expr,
        many0(pair(symbol(","), sequence_match_item)),
    ))(s)?;
    Ok((
        s,
        SequenceExpr::FirstMatch(Box::new(SequenceExprFirstMatch { nodes: (a, b) })),
    ))
}

#[parser(MaybeRecursive)]
pub fn sequence_expr_throughout(s: Span) -> IResult<Span, SequenceExpr> {
    let (s, a) = expression_or_dist(s)?;
    let (s, b) = keyword("throughout")(s)?;
    let (s, c) = sequence_expr(s)?;
    Ok((
        s,
        SequenceExpr::Throughout(Box::new(SequenceExprThroughout { nodes: (a, b, c) })),
    ))
}

#[parser(MaybeRecursive)]
pub fn sequence_expr_within(s: Span) -> IResult<Span, SequenceExpr> {
    let (s, a) = sequence_expr(s)?;
    let (s, b) = keyword("within")(s)?;
    let (s, c) = sequence_expr(s)?;
    Ok((
        s,
        SequenceExpr::Within(Box::new(SequenceExprWithin { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn sequence_expr_clocking_event(s: Span) -> IResult<Span, SequenceExpr> {
    let (s, a) = clocking_event(s)?;
    let (s, b) = sequence_expr(s)?;
    Ok((
        s,
        SequenceExpr::ClockingEvent(Box::new(SequenceExprClockingEvent { nodes: (a, b) })),
    ))
}

#[parser]
pub fn cycle_delay_range(s: Span) -> IResult<Span, CycleDelayRange> {
    alt((
        cycle_delay_range_primary,
        cycle_delay_range_expression,
        cycle_delay_range_asterisk,
        cycle_delay_range_plus,
    ))(s)
}

#[parser]
pub fn cycle_delay_range_primary(s: Span) -> IResult<Span, CycleDelayRange> {
    let (s, a) = symbol("##")(s)?;
    let (s, b) = constant_primary(s)?;
    Ok((
        s,
        CycleDelayRange::Primary(CycleDelayRangePrimary { nodes: (a, b) }),
    ))
}

#[parser]
pub fn cycle_delay_range_expression(s: Span) -> IResult<Span, CycleDelayRange> {
    let (s, a) = symbol("##")(s)?;
    let (s, b) = bracket(cycle_delay_const_range_expression)(s)?;
    Ok((
        s,
        CycleDelayRange::Expression(CycleDelayRangeExpression { nodes: (a, b) }),
    ))
}

#[parser]
pub fn cycle_delay_range_asterisk(s: Span) -> IResult<Span, CycleDelayRange> {
    let (s, a) = symbol("##")(s)?;
    let (s, b) = bracket(symbol("*"))(s)?;
    Ok((
        s,
        CycleDelayRange::Asterisk(CycleDelayRangeAsterisk { nodes: (a, b) }),
    ))
}

#[parser]
pub fn cycle_delay_range_plus(s: Span) -> IResult<Span, CycleDelayRange> {
    let (s, a) = symbol("##")(s)?;
    let (s, b) = bracket(symbol("+"))(s)?;
    Ok((
        s,
        CycleDelayRange::Plus(CycleDelayRangePlus { nodes: (a, b) }),
    ))
}

#[parser(Memoize)]
pub fn sequence_method_call(s: Span) -> IResult<Span, SequenceMethodCall> {
    let (s, a) = sequence_instance(s)?;
    let (s, b) = symbol(".")(s)?;
    let (s, c) = method_identifier(s)?;
    Ok((s, SequenceMethodCall { nodes: (a, b, c) }))
}

#[parser]
pub fn sequence_match_item(s: Span) -> IResult<Span, SequenceMatchItem> {
    alt((
        map(operator_assignment, |x| {
            SequenceMatchItem::OperatorAssignment(x)
        }),
        map(inc_or_dec_expression, |x| {
            SequenceMatchItem::IncOrDecExpression(x)
        }),
        map(subroutine_call, |x| SequenceMatchItem::SubroutineCall(x)),
    ))(s)
}

#[parser]
pub fn sequence_instance(s: Span) -> IResult<Span, SequenceInstance> {
    let (s, a) = ps_or_hierarchical_sequence_identifier(s)?;
    let (s, b) = opt(paren(opt(sequence_list_of_arguments)))(s)?;
    Ok((s, SequenceInstance { nodes: (a, b) }))
}

#[parser]
pub fn sequence_list_of_arguments(s: Span) -> IResult<Span, SequenceListOfArguments> {
    alt((
        sequence_list_of_arguments_ordered,
        sequence_list_of_arguments_named,
    ))(s)
}

#[parser(MaybeRecursive)]
pub fn sequence_list_of_arguments_ordered(s: Span) -> IResult<Span, SequenceListOfArguments> {
    let (s, a) = list(symbol(","), opt(sequence_actual_arg))(s)?;
    let (s, b) = many0(tuple((
        symbol(","),
        symbol("."),
        identifier,
        paren(opt(sequence_actual_arg)),
    )))(s)?;
    Ok((
        s,
        SequenceListOfArguments::Ordered(SequenceListOfArgumentsOrdered { nodes: (a, b) }),
    ))
}

#[parser]
pub fn sequence_list_of_arguments_named(s: Span) -> IResult<Span, SequenceListOfArguments> {
    let (s, a) = list(
        symbol(","),
        triple(symbol("."), identifier, paren(opt(sequence_actual_arg))),
    )(s)?;
    Ok((
        s,
        SequenceListOfArguments::Named(SequenceListOfArgumentsNamed { nodes: (a,) }),
    ))
}

#[parser]
pub fn sequence_actual_arg(s: Span) -> IResult<Span, SequenceActualArg> {
    alt((
        map(event_expression, |x| SequenceActualArg::EventExpression(x)),
        map(sequence_expr, |x| SequenceActualArg::SequenceExpr(x)),
    ))(s)
}

#[parser]
pub fn boolean_abbrev(s: Span) -> IResult<Span, BooleanAbbrev> {
    alt((
        map(consecutive_repetition, |x| {
            BooleanAbbrev::ConsecutiveRepetition(x)
        }),
        map(non_consecutive_repetition, |x| {
            BooleanAbbrev::NonConsecutiveRepetition(x)
        }),
        map(goto_repetition, |x| BooleanAbbrev::GotoRepetition(x)),
    ))(s)
}

#[parser]
pub fn sequence_abbrev(s: Span) -> IResult<Span, SequenceAbbrev> {
    let (s, a) = consecutive_repetition(s)?;
    Ok((s, SequenceAbbrev { nodes: (a,) }))
}

#[parser]
pub fn consecutive_repetition(s: Span) -> IResult<Span, ConsecutiveRepetition> {
    alt((
        consecutive_repetition_expression,
        consecutive_repetition_asterisk,
        consecutive_repetition_plus,
    ))(s)
}

#[parser]
pub fn consecutive_repetition_expression(s: Span) -> IResult<Span, ConsecutiveRepetition> {
    let (s, a) = bracket(pair(symbol("*"), const_or_range_expression))(s)?;
    Ok((
        s,
        ConsecutiveRepetition::Expression(ConsecutiveRepetitionExpression { nodes: (a,) }),
    ))
}

#[parser]
pub fn consecutive_repetition_asterisk(s: Span) -> IResult<Span, ConsecutiveRepetition> {
    let (s, a) = bracket(symbol("*"))(s)?;
    Ok((
        s,
        ConsecutiveRepetition::Asterisk(ConsecutiveRepetitionAsterisk { nodes: (a,) }),
    ))
}

#[parser]
pub fn consecutive_repetition_plus(s: Span) -> IResult<Span, ConsecutiveRepetition> {
    let (s, a) = bracket(symbol("+"))(s)?;
    Ok((
        s,
        ConsecutiveRepetition::Plus(ConsecutiveRepetitionPlus { nodes: (a,) }),
    ))
}

#[parser]
pub fn non_consecutive_repetition(s: Span) -> IResult<Span, NonConsecutiveRepetition> {
    let (s, a) = bracket(pair(symbol("="), const_or_range_expression))(s)?;
    Ok((s, NonConsecutiveRepetition { nodes: (a,) }))
}

#[parser]
pub fn goto_repetition(s: Span) -> IResult<Span, GotoRepetition> {
    let (s, a) = bracket(pair(symbol("->"), const_or_range_expression))(s)?;
    Ok((s, GotoRepetition { nodes: (a,) }))
}

#[parser]
pub fn const_or_range_expression(s: Span) -> IResult<Span, ConstOrRangeExpression> {
    alt((
        map(constant_expression, |x| {
            ConstOrRangeExpression::ConstantExpression(x)
        }),
        map(cycle_delay_const_range_expression, |x| {
            ConstOrRangeExpression::CycleDelayConstRangeExpression(x)
        }),
    ))(s)
}

#[parser]
pub fn cycle_delay_const_range_expression(
    s: Span,
) -> IResult<Span, CycleDelayConstRangeExpression> {
    alt((
        cycle_delay_const_range_expression_binary,
        cycle_delay_const_range_expression_dollar,
    ))(s)
}

#[parser(MaybeRecursive)]
pub fn cycle_delay_const_range_expression_binary(
    s: Span,
) -> IResult<Span, CycleDelayConstRangeExpression> {
    let (s, a) = constant_expression(s)?;
    let (s, b) = symbol(":")(s)?;
    let (s, c) = constant_expression(s)?;
    Ok((
        s,
        CycleDelayConstRangeExpression::Binary(CycleDelayConstRangeExpressionBinary {
            nodes: (a, b, c),
        }),
    ))
}

#[parser(MaybeRecursive)]
pub fn cycle_delay_const_range_expression_dollar(
    s: Span,
) -> IResult<Span, CycleDelayConstRangeExpression> {
    let (s, a) = constant_expression(s)?;
    let (s, b) = symbol(":")(s)?;
    let (s, c) = symbol("$")(s)?;
    Ok((
        s,
        CycleDelayConstRangeExpression::Dollar(CycleDelayConstRangeExpressionDollar {
            nodes: (a, b, c),
        }),
    ))
}

#[parser(MaybeRecursive)]
pub fn expression_or_dist(s: Span) -> IResult<Span, ExpressionOrDist> {
    let (s, a) = expression(s)?;
    let (s, b) = opt(pair(keyword("dist"), brace(dist_list)))(s)?;
    Ok((s, ExpressionOrDist { nodes: (a, b) }))
}

#[parser]
pub fn assertion_variable_declaration(s: Span) -> IResult<Span, AssertionVariableDeclaration> {
    let (s, a) = var_data_type(s)?;
    let (s, b) = list_of_variable_decl_assignments(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, AssertionVariableDeclaration { nodes: (a, b, c) }))
}