use crate::ast::*;
use crate::parser::*;
use nom::branch::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Node)]
pub enum DataDeclaration {
    Variable(Box<DataDeclarationVariable>),
    TypeDeclaration(Box<TypeDeclaration>),
    PackageImportDeclaration(Box<PackageImportDeclaration>),
    NetTypeDeclaration(Box<NetTypeDeclaration>),
}

#[derive(Clone, Debug, Node)]
pub struct DataDeclarationVariable {
    pub nodes: (
        Option<Const>,
        Option<Var>,
        Option<Lifetime>,
        Option<DataTypeOrImplicit>,
        ListOfVariableDeclAssignments,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct Const {
    pub nodes: (Keyword,),
}

#[derive(Clone, Debug, Node)]
pub struct PackageImportDeclaration {
    pub nodes: (Keyword, List<Symbol, PackageImportItem>, Symbol),
}

#[derive(Clone, Debug, Node)]
pub enum PackageImportItem {
    Identifier(Box<PackageImportItemIdentifier>),
    Asterisk(Box<PackageImportItemAsterisk>),
}

#[derive(Clone, Debug, Node)]
pub struct PackageImportItemIdentifier {
    pub nodes: (PackageIdentifier, Symbol, Identifier),
}

#[derive(Clone, Debug, Node)]
pub struct PackageImportItemAsterisk {
    pub nodes: (PackageIdentifier, Symbol, Symbol),
}

#[derive(Clone, Debug, Node)]
pub enum PackageExportDeclaration {
    Asterisk(Box<PackageExportDeclarationAsterisk>),
    Item(Box<PackageExportDeclarationItem>),
}

#[derive(Clone, Debug, Node)]
pub struct PackageExportDeclarationAsterisk {
    pub nodes: (Keyword, Symbol, Symbol),
}

#[derive(Clone, Debug, Node)]
pub struct PackageExportDeclarationItem {
    pub nodes: (Keyword, List<Symbol, PackageImportItem>, Symbol),
}

#[derive(Clone, Debug, Node)]
pub struct GenvarDeclaration {
    pub nodes: (Keyword, ListOfGenvarIdentifiers, Symbol),
}

#[derive(Clone, Debug, Node)]
pub enum NetDeclaration {
    NetType(Box<NetDeclarationNetType>),
    NetTypeIdentifier(Box<NetDeclarationNetTypeIdentifier>),
    Interconnect(Box<NetDeclarationInterconnect>),
}

#[derive(Clone, Debug, Node)]
pub struct NetDeclarationNetType {
    pub nodes: (
        NetType,
        Option<Strength>,
        Option<VectorScalar>,
        Option<DataTypeOrImplicit>,
        Option<Delay3>,
        ListOfNetDeclAssignments,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub enum Strength {
    Drive(Box<DriveStrength>),
    Charge(Box<ChargeStrength>),
}

#[derive(Clone, Debug, Node)]
pub enum VectorScalar {
    Vectored(Box<Keyword>),
    Scalared(Box<Keyword>),
}

#[derive(Clone, Debug, Node)]
pub struct NetDeclarationNetTypeIdentifier {
    pub nodes: (
        NetTypeIdentifier,
        Option<DelayControl>,
        ListOfNetDeclAssignments,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct NetDeclarationInterconnect {
    pub nodes: (
        Keyword,
        ImplicitDataType,
        Option<(Symbol, DelayValue)>,
        NetIdentifier,
        Vec<UnpackedDimension>,
        Option<(Symbol, NetIdentifier, Vec<UnpackedDimension>)>,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub enum TypeDeclaration {
    DataType(Box<TypeDeclarationDataType>),
    Interface(Box<TypeDeclarationInterface>),
    Reserved(Box<TypeDeclarationReserved>),
}

#[derive(Clone, Debug, Node)]
pub struct TypeDeclarationDataType {
    pub nodes: (
        Keyword,
        DataType,
        TypeIdentifier,
        Vec<VariableDimension>,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct TypeDeclarationInterface {
    pub nodes: (
        Keyword,
        InterfaceInstanceIdentifier,
        ConstantBitSelect,
        Symbol,
        TypeIdentifier,
        TypeIdentifier,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct TypeDeclarationReserved {
    pub nodes: (
        Keyword,
        Option<TypeDeclarationKeyword>,
        TypeIdentifier,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub enum TypeDeclarationKeyword {
    Enum(Box<Keyword>),
    Struct(Box<Keyword>),
    Union(Box<Keyword>),
    Class(Box<Keyword>),
    InterfaceClass(Box<(Keyword, Keyword)>),
}

#[derive(Clone, Debug, Node)]
pub enum NetTypeDeclaration {
    DataType(Box<NetTypeDeclarationDataType>),
    NetType(Box<NetTypeDeclarationNetType>),
}

#[derive(Clone, Debug, Node)]
pub struct NetTypeDeclarationDataType {
    pub nodes: (
        Keyword,
        DataType,
        NetTypeIdentifier,
        Option<(Keyword, Option<PackageScopeOrClassScope>, TfIdentifier)>,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct NetTypeDeclarationNetType {
    pub nodes: (
        Keyword,
        Option<PackageScopeOrClassScope>,
        NetTypeIdentifier,
        NetTypeIdentifier,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub enum Lifetime {
    Static(Box<Keyword>),
    Automatic(Box<Keyword>),
}

// -----------------------------------------------------------------------------

#[parser]
pub(crate) fn data_declaration(s: Span) -> IResult<Span, DataDeclaration> {
    alt((
        data_declaration_variable,
        map(type_declaration, |x| {
            DataDeclaration::TypeDeclaration(Box::new(x))
        }),
        map(package_import_declaration, |x| {
            DataDeclaration::PackageImportDeclaration(Box::new(x))
        }),
        map(net_type_declaration, |x| {
            DataDeclaration::NetTypeDeclaration(Box::new(x))
        }),
    ))(s)
}

#[parser(Ambiguous)]
pub(crate) fn data_declaration_variable(s: Span) -> IResult<Span, DataDeclaration> {
    let (s, a) = opt(r#const)(s)?;
    let (s, b) = opt(var)(s)?;
    let (s, c) = opt(lifetime)(s)?;
    let (s, d) = ambiguous_opt(data_type_or_implicit)(s)?;
    let (s, e) = list_of_variable_decl_assignments(s)?;
    let (s, f) = symbol(";")(s)?;
    Ok((
        s,
        DataDeclaration::Variable(Box::new(DataDeclarationVariable {
            nodes: (a, b, c, d, e, f),
        })),
    ))
}

#[parser]
pub(crate) fn r#const(s: Span) -> IResult<Span, Const> {
    let (s, a) = keyword("const")(s)?;
    Ok((s, Const { nodes: (a,) }))
}

#[parser]
pub(crate) fn package_import_declaration(s: Span) -> IResult<Span, PackageImportDeclaration> {
    let (s, a) = keyword("import")(s)?;
    let (s, b) = list(symbol(","), package_import_item)(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, PackageImportDeclaration { nodes: (a, b, c) }))
}

#[parser]
pub(crate) fn package_import_item(s: Span) -> IResult<Span, PackageImportItem> {
    alt((package_import_item_identifier, package_import_item_asterisk))(s)
}

#[parser]
pub(crate) fn package_import_item_identifier(s: Span) -> IResult<Span, PackageImportItem> {
    let (s, a) = package_identifier(s)?;
    let (s, b) = symbol("::")(s)?;
    let (s, c) = identifier(s)?;
    Ok((
        s,
        PackageImportItem::Identifier(Box::new(PackageImportItemIdentifier { nodes: (a, b, c) })),
    ))
}

#[parser]
pub(crate) fn package_import_item_asterisk(s: Span) -> IResult<Span, PackageImportItem> {
    let (s, a) = package_identifier(s)?;
    let (s, b) = symbol("::")(s)?;
    let (s, c) = symbol("*")(s)?;
    Ok((
        s,
        PackageImportItem::Asterisk(Box::new(PackageImportItemAsterisk { nodes: (a, b, c) })),
    ))
}

#[parser]
pub(crate) fn package_export_declaration(s: Span) -> IResult<Span, PackageExportDeclaration> {
    alt((
        package_export_declaration_asterisk,
        package_export_declaration_item,
    ))(s)
}

#[parser]
pub(crate) fn package_export_declaration_asterisk(s: Span) -> IResult<Span, PackageExportDeclaration> {
    let (s, a) = keyword("export")(s)?;
    let (s, b) = symbol("*::*")(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((
        s,
        PackageExportDeclaration::Asterisk(Box::new(PackageExportDeclarationAsterisk {
            nodes: (a, b, c),
        })),
    ))
}

#[parser]
pub(crate) fn package_export_declaration_item(s: Span) -> IResult<Span, PackageExportDeclaration> {
    let (s, a) = keyword("export")(s)?;
    let (s, b) = list(symbol(","), package_import_item)(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((
        s,
        PackageExportDeclaration::Item(Box::new(PackageExportDeclarationItem { nodes: (a, b, c) })),
    ))
}

#[parser]
pub(crate) fn genvar_declaration(s: Span) -> IResult<Span, GenvarDeclaration> {
    let (s, a) = keyword("genvar")(s)?;
    let (s, b) = list_of_genvar_identifiers(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, GenvarDeclaration { nodes: (a, b, c) }))
}

#[parser]
pub(crate) fn net_declaration(s: Span) -> IResult<Span, NetDeclaration> {
    alt((
        net_declaration_interconnect,
        net_declaration_net_type,
        net_declaration_net_type_identifier,
    ))(s)
}

#[parser(Ambiguous)]
pub(crate) fn net_declaration_net_type(s: Span) -> IResult<Span, NetDeclaration> {
    let (s, a) = net_type(s)?;
    let (s, b) = opt(strength)(s)?;
    let (s, c) = opt(vector_scalar)(s)?;
    let (s, d) = ambiguous_opt(data_type_or_implicit)(s)?;
    let (s, e) = opt(delay3)(s)?;
    let (s, f) = list_of_net_decl_assignments(s)?;
    let (s, g) = symbol(";")(s)?;
    Ok((
        s,
        NetDeclaration::NetType(Box::new(NetDeclarationNetType {
            nodes: (a, b, c, d, e, f, g),
        })),
    ))
}

#[parser]
pub(crate) fn strength(s: Span) -> IResult<Span, Strength> {
    alt((
        map(drive_strength, |x| Strength::Drive(Box::new(x))),
        map(charge_strength, |x| Strength::Charge(Box::new(x))),
    ))(s)
}

#[parser]
pub(crate) fn vector_scalar(s: Span) -> IResult<Span, VectorScalar> {
    alt((
        map(keyword("vectored"), |x| VectorScalar::Vectored(Box::new(x))),
        map(keyword("scalared"), |x| VectorScalar::Scalared(Box::new(x))),
    ))(s)
}

#[parser]
pub(crate) fn net_declaration_net_type_identifier(s: Span) -> IResult<Span, NetDeclaration> {
    let (s, a) = net_type_identifier(s)?;
    let (s, b) = opt(delay_control)(s)?;
    let (s, c) = list_of_net_decl_assignments(s)?;
    let (s, d) = symbol(";")(s)?;
    Ok((
        s,
        NetDeclaration::NetTypeIdentifier(Box::new(NetDeclarationNetTypeIdentifier {
            nodes: (a, b, c, d),
        })),
    ))
}

#[parser]
pub(crate) fn net_declaration_interconnect(s: Span) -> IResult<Span, NetDeclaration> {
    let (s, a) = keyword("interconnect")(s)?;
    let (s, b) = implicit_data_type(s)?;
    let (s, c) = opt(pair(symbol("#"), delay_value))(s)?;
    let (s, d) = net_identifier(s)?;
    let (s, e) = many0(unpacked_dimension)(s)?;
    let (s, f) = opt(triple(
        symbol(","),
        net_identifier,
        many0(unpacked_dimension),
    ))(s)?;
    let (s, g) = symbol(";")(s)?;
    Ok((
        s,
        NetDeclaration::Interconnect(Box::new(NetDeclarationInterconnect {
            nodes: (a, b, c, d, e, f, g),
        })),
    ))
}

#[parser]
pub(crate) fn type_declaration(s: Span) -> IResult<Span, TypeDeclaration> {
    alt((
        type_declaration_data_type,
        type_declaration_interface,
        type_declaration_reserved,
    ))(s)
}

#[parser]
pub(crate) fn type_declaration_data_type(s: Span) -> IResult<Span, TypeDeclaration> {
    let (s, a) = keyword("typedef")(s)?;
    let (s, b) = data_type(s)?;
    let (s, c) = type_identifier(s)?;
    let (s, d) = many0(variable_dimension)(s)?;
    let (s, e) = symbol(";")(s)?;
    Ok((
        s,
        TypeDeclaration::DataType(Box::new(TypeDeclarationDataType {
            nodes: (a, b, c, d, e),
        })),
    ))
}

#[parser]
pub(crate) fn type_declaration_interface(s: Span) -> IResult<Span, TypeDeclaration> {
    let (s, a) = keyword("typedef")(s)?;
    let (s, b) = interface_instance_identifier(s)?;
    let (s, c) = constant_bit_select(s)?;
    let (s, d) = symbol(".")(s)?;
    let (s, e) = type_identifier(s)?;
    let (s, f) = type_identifier(s)?;
    let (s, g) = symbol(";")(s)?;
    Ok((
        s,
        TypeDeclaration::Interface(Box::new(TypeDeclarationInterface {
            nodes: (a, b, c, d, e, f, g),
        })),
    ))
}

#[parser]
pub(crate) fn type_declaration_reserved(s: Span) -> IResult<Span, TypeDeclaration> {
    let (s, a) = keyword("typedef")(s)?;
    let (s, b) = opt(type_declaration_keyword)(s)?;
    let (s, c) = type_identifier(s)?;
    let (s, d) = symbol(";")(s)?;
    Ok((
        s,
        TypeDeclaration::Reserved(Box::new(TypeDeclarationReserved {
            nodes: (a, b, c, d),
        })),
    ))
}

#[parser]
pub(crate) fn type_declaration_keyword(s: Span) -> IResult<Span, TypeDeclarationKeyword> {
    alt((
        map(keyword("enum"), |x| {
            TypeDeclarationKeyword::Enum(Box::new(x))
        }),
        map(keyword("struct"), |x| {
            TypeDeclarationKeyword::Struct(Box::new(x))
        }),
        map(keyword("union"), |x| {
            TypeDeclarationKeyword::Union(Box::new(x))
        }),
        map(keyword("class"), |x| {
            TypeDeclarationKeyword::Class(Box::new(x))
        }),
        map(pair(keyword("interface"), keyword("class")), |x| {
            TypeDeclarationKeyword::InterfaceClass(Box::new(x))
        }),
    ))(s)
}

#[parser]
pub(crate) fn net_type_declaration(s: Span) -> IResult<Span, NetTypeDeclaration> {
    alt((
        net_type_declaration_data_type,
        net_type_declaration_net_type,
    ))(s)
}

#[parser]
pub(crate) fn net_type_declaration_data_type(s: Span) -> IResult<Span, NetTypeDeclaration> {
    let (s, a) = keyword("nettype")(s)?;
    let (s, b) = data_type(s)?;
    let (s, c) = net_type_identifier(s)?;
    let (s, d) = opt(triple(
        keyword("with"),
        opt(package_scope_or_class_scope),
        tf_identifier,
    ))(s)?;
    let (s, e) = symbol(";")(s)?;
    Ok((
        s,
        NetTypeDeclaration::DataType(Box::new(NetTypeDeclarationDataType {
            nodes: (a, b, c, d, e),
        })),
    ))
}

#[parser]
pub(crate) fn net_type_declaration_net_type(s: Span) -> IResult<Span, NetTypeDeclaration> {
    let (s, a) = keyword("nettype")(s)?;
    let (s, b) = opt(package_scope_or_class_scope)(s)?;
    let (s, c) = net_type_identifier(s)?;
    let (s, d) = net_type_identifier(s)?;
    let (s, e) = symbol(";")(s)?;
    Ok((
        s,
        NetTypeDeclaration::NetType(Box::new(NetTypeDeclarationNetType {
            nodes: (a, b, c, d, e),
        })),
    ))
}

#[parser]
pub(crate) fn lifetime(s: Span) -> IResult<Span, Lifetime> {
    alt((
        map(keyword("static"), |x| Lifetime::Static(Box::new(x))),
        map(keyword("automatic"), |x| Lifetime::Automatic(Box::new(x))),
    ))(s)
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_net_type_declaration() {
        parser_test!(
            net_type_declaration,
            "nettype T wT;",
            Ok((_, NetTypeDeclaration::DataType(_)))
        );
        parser_test!(
            net_type_declaration,
            "nettype T wTsum with Tsum;",
            Ok((_, NetTypeDeclaration::DataType(_)))
        );
        parser_test!(
            net_type_declaration,
            "nettype MyBaseT::T narrowTsum with MyBaseT::Tsum;",
            Ok((_, NetTypeDeclaration::DataType(_)))
        );
    }

    #[test]
    fn test_net_declaration() {
        parser_test!(
            net_declaration,
            "trireg (large) logic #(0,0,0) cap1;",
            Ok((_, NetDeclaration::NetType(_)))
        );
        parser_test!(
            net_declaration,
            "wire addressT w1;",
            Ok((_, NetDeclaration::NetType(_)))
        );
        parser_test!(
            net_declaration,
            "wire struct packed { logic ecc; logic [7:0] data; } memsig;",
            Ok((_, NetDeclaration::NetType(_)))
        );
        parser_test!(
            net_declaration,
            "wire w;",
            Ok((_, NetDeclaration::NetType(_)))
        );
        parser_test!(
            net_declaration,
            "wire [15:0] w;",
            Ok((_, NetDeclaration::NetType(_)))
        );
        parser_test!(
            net_declaration,
            "interconnect w1;",
            Ok((_, NetDeclaration::Interconnect(_)))
        );
        parser_test!(
            net_declaration,
            "interconnect [3:0] w2;",
            Ok((_, NetDeclaration::Interconnect(_)))
        );
        parser_test!(
            net_declaration,
            "interconnect [3:0] w3 [1:0];",
            Ok((_, NetDeclaration::Interconnect(_)))
        );
        parser_test!(net_declaration, "interconnect logic [3:0] w4;", Err(_));
        parser_test!(net_declaration, "interconnect #(1,2,3) w5;", Err(_));
        parser_test!(
            net_declaration,
            "wand w;",
            Ok((_, NetDeclaration::NetType(_)))
        );
        parser_test!(
            net_declaration,
            "tri [15:0] busa;",
            Ok((_, NetDeclaration::NetType(_)))
        );
        parser_test!(
            net_declaration,
            "trireg (small) storeit;",
            Ok((_, NetDeclaration::NetType(_)))
        );
        parser_test!(
            net_declaration,
            "wire w1, w2;",
            Ok((_, NetDeclaration::NetType(_)))
        );
        parser_test!(
            net_declaration,
            "tri1 scalared [63:0] bus64;",
            Ok((_, NetDeclaration::NetType(_)))
        );
        parser_test!(
            net_declaration,
            "tri vectored [31:0] data;",
            Ok((_, NetDeclaration::NetType(_)))
        );
    }

    #[test]
    fn test_data_declaration() {
        //parser_test!(
        //    data_declaration,
        //    "shortint s1, s2[0:9];",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "var byte my_byte;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "var v;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "var [15:0] vw;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "var enum bit { clear, error } status;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "var reg r;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "int i = 0;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "logic a;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "logic[3:0] v;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "logic signed [3:0] signed_reg;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "logic [-1:4] b;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "logic [4:0] x, y, z;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "int unsigned ui;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "int signed si;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "string myName = default_name;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "byte c = \"A\";",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "bit [10:0] b = \"x41\";",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "bit [1:4][7:0] h = \"hello\" ;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "event done;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "event done_too = done;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "event empty = null;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "typedef int intP;",
        //    Ok((_, DataDeclaration::TypeDeclaration(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "intP a, b;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "typedef enum type_identifier;",
        //    Ok((_, DataDeclaration::TypeDeclaration(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "typedef struct type_identifier;",
        //    Ok((_, DataDeclaration::TypeDeclaration(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "typedef union type_identifier;",
        //    Ok((_, DataDeclaration::TypeDeclaration(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "typedef class type_identifier;",
        //    Ok((_, DataDeclaration::TypeDeclaration(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "typedef interface class type_identifier;",
        //    Ok((_, DataDeclaration::TypeDeclaration(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "typedef type_identifier;",
        //    Ok((_, DataDeclaration::TypeDeclaration(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "typedef C::T c_t;",
        //    Ok((_, DataDeclaration::TypeDeclaration(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "enum {red, yellow, green} light1, light2;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "enum bit [1:0] {IDLE, XX='x, S1=2'b01, S2=2'b10} state, next;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "enum integer {IDLE, XX='x, S1='b01, S2='b10} state, next;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "enum integer {IDLE, XX='x, S1='b01, S2='b10} state, next;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "enum {bronze=3, silver, gold} medal;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "enum {a=3, b=7, c} alphabet;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "enum bit [3:0] {bronze='h3, silver, gold='h5} medal2;",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "integer i_array[*];",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "bit [20:0] array_b[string];",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "event ev_array[myClass];",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "int array_name [*];",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "int array_name1 [ integer ];",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "int a[int] = '{default:1};",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        //parser_test!(
        //    data_declaration,
        //    "byte q1[$];",
        //    Ok((_, DataDeclaration::Variable(_)))
        //);
        parser_test!(primary, "'{default:1}", Ok((_, _)));
    }
}