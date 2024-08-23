
enum LongIdent {
    Lident(String),
    Ldot(Box<LongIdent>, String),
    Lapply(Box<LongIdent>, Box<LongIdent>),
}

enum CoreTypeDesc {
    PtypConstr(LongIdent, Vec<CoreType>),
    // ... other variants ...
}

struct CoreType {
    desc: CoreTypeDesc,
    location: Location,
    location_stack: Vec<Location>,
    attributes: Vec<Attribute>,
}

enum ConstructorArguments {
    Tuple(Vec<CoreType>),  // Tuple style constructor arguments (Pcstr_tuple)
    Record(Vec<LabelDeclaration>),  // Record style constructor arguments (Pcstr_record)
}

struct ConstructorDeclaration {
    name: String,
    vars: Vec<String>,
    args: ConstructorArguments,
    res: Option<CoreType>,
    location: Location,
    attributes: Vec<Attribute>,
}
// constructor_declaration =
//      pcd_name: string loc;
//      pcd_vars: string loc list;
//      pcd_args: constructor_arguments;
//      pcd_res: core_type option;
//      pcd_loc: Location.t;
//      pcd_attributes: attributes;  (** [C of ... [\@id1] [\@id2]] *)


enum TypeKind {
    Abstract, //Ptype_abstract,
    Variant(Vec<ConstructorDeclaration>), // Ptype_v
    Record(Vec<LabelDeclaration>),//  (** Invariant: non-empty list *) Ptype_record
    Open //Ptype_open
}

struct Location{}
enum Variance  {
    Covariant,
    Contravariant,
    NoVariance
}

enum Injectivity {
    Injective,
    NoInjectivity,
}
struct TypeDeclaration {
    name: String,
    params: Vec<(String, Variance, Injectivity)>,
    constraints: Vec<(String, String, Location)>,
    kind: TypeKind,
//    private: PrivateFlag,
    manifest: Option<String>,
    attributes: Vec<String>,
    location: Location,
}

enum MutableFlag {
    Immutable,
    Mmutable
}

struct LabelDeclaration {
    name: String,
    mutable: MutableFlag,
    type_: CoreType,
    location: Location,
    attributes: Vec<Attribute>,
}

struct StructureItem {
    desc: StructureItemDesc,
    location: Location,
}

enum StructureItemDesc {
    PstrEval(Expression, Vec<Attribute>),
}

struct Expression {
    desc: ExpressionDesc,
    location: Location,
    location_stack: Vec<Location>,
    attributes: Vec<Attribute>,
}

enum ExpressionDesc {
    PexpConstant(Constant),
}

struct Constant {
    desc: ConstantDesc,
    location: Location,
}

enum ConstantDesc {
    PconstString(String, Location, Option<String>),
}

type Structure = Vec<StructureItem>;
type Signature = Vec<SignatureItem>;

struct SignatureItem {
    desc: SignatureItemDesc,
    location: Location,
}


enum RecFlag {
    Recursive,
    NonRecursive,
}

enum Payload {
    PStr(Structure),
    PSig(Signature),
    PTyp(CoreType),
//    PPat(Pattern, Option<Expression>),
}

struct Attribute {
    name: String,
    payload: Payload,
    location: Location,
}

struct ModuleExpr {
    desc: ModuleExprDesc,
    location: Location,
    attributes: Vec<Attribute>,
}

enum ModuleExprDesc {
    PmodIdent,
    PmodStructure,
    PmodFunctor,
    PmodApply,
    PmodApplyUnit,
    PmodConstraint,
    PmodUnpack,
    PmodExtension,
}

struct OpenInfos {
    expr: String,
//    override_flag: OverrideFlag,
    location: Location,
    attributes: Vec<Attribute>,
}

struct OpenDescription {
    longident: LongIdent,
    open_infos: OpenInfos,
}


enum SignatureItemDesc {
    PsigType(RecFlag, Vec<TypeDeclaration>),
    PsigOpen(OpenDescription),
    PsigAttribute(Attribute),
}


// Implement AstDeserializer trait for each structure

trait AstDeserializer {
    fn deserialize(ast_json: &str) -> Self;
}


fn main() {
    println!("Hello, world!");
}
