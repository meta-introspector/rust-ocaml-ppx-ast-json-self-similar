mod deserialize;
mod visitor;
//crate ::deserialize;

//use crate::deserialize;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, serde::Serialize)]
pub enum LongIdent {
    Lident(String),
    Ldot(Box<LongIdent>, String),
    Lapply(Box<LongIdent>, Box<LongIdent>),
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub enum CoreTypeDesc {
    // Ptype_constr
    PtypConstr(LongIdent, Vec<CoreType>),
    // ... other variants ...
    None
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
struct LocString {
    //loc2: {
    txt2: String
}

#[derive(serde::Deserialize, Debug)]
    #[allow(dead_code)]
pub struct CoreType {
    #[serde(rename = "ptyp_desc")]
    #[serde(deserialize_with = "deserialize::deserialize_core_type_desc")]
    desc: CoreTypeDesc, // ptyp_
    //    location: Location,
    //    location_stack: Vec<Location>,
    #[serde(rename = "ptyp_attributes")]
    attributes: Vec<Attribute>,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub enum ConstructorArguments {
    Tuple(Vec<CoreType>),          // Tuple style constructor arguments (Pcstr_tuple)
    Record(Vec<LabelDeclaration>), // Record style constructor arguments (Pcstr_record)
    None
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct ConstructorDeclaration {
    #[serde(rename = "pcd_name")]
    name: LocString,
    #[serde(rename = "pcd_vars")]
    vars: Vec<String>,    
    #[serde(rename = "pcd_args")]
    #[serde(deserialize_with = "deserialize::deserialize_constructor_arguments")]
    args: ConstructorArguments,
    #[serde(rename = "pcd_res")]
    res: Option<CoreType>,   
    //    location: Location,
    #[serde(rename = "pcd_attributes")]
    attributes: Vec<Attribute>,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub enum TypeKind {
    Abstract,                             //Ptype_abstract,
    Variant(Vec<ConstructorDeclaration>), // Ptype_v
    Record(Vec<LabelDeclaration>),        //  (** Invariant: non-empty list *) Ptype_record
    Open,                                 //Ptype_open
    None
}

#[derive(serde::Deserialize, Debug)]
pub struct Location {}

#[derive(serde::Deserialize, Debug)]
pub enum Variance {
    Covariant,
    Contravariant,
    NoVariance,
}

#[derive(serde::Deserialize, Debug)]
pub enum Injectivity {
    Injective,
    NoInjectivity,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct TypeDeclaration {
    #[serde(rename = "ptype_name")]
    name: LocString,
    #[serde(rename = "ptype_params")]
    params: Vec<(String, Variance, Injectivity)>,
    #[serde(rename = "ptype_cstrs")]
    constraints: Vec<(String, String, Location)>,
    #[serde(rename = "ptype_kind")]
    #[serde(deserialize_with = "deserialize::deserialize_ptype_kind")]
    kind: TypeKind,

    //    #[serde(rename = "ptype_private")]
    //    private: PrivateFlag,
    #[serde(rename = "ptype_manifest")]
    manifest: Option<String>,
    #[serde(rename = "ptype_attributes")]
    attributes: Vec<String>,
    //#[serde(rename = "ptype_loc")]
    //location: Location,
}

#[derive(serde::Deserialize, Debug)]
pub enum MutableFlag {
    Immutable,
    Mmutable,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct LabelDeclaration {
    #[serde(rename = "pld_name")]
    //pld_name1
    name: LocString,
    #[serde(rename = "pld_mutable")]
    mutable: Vec<MutableFlag>,
    #[serde(rename = "pld_type")]
    type_: CoreType,   
    //location: Location,
    #[serde(rename = "pld_attributes")]
    attributes: Vec<Attribute>,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct StructureItem {
    //     pstr_desc
    #[serde(rename = "pstr_desc")]
    desc: StructureItemDesc,
    //location: Location,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub enum StructureItemDesc {
    PstrEval(Expression, Vec<Attribute>),
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct Expression {
    //     pexp_desc: expression_desc;
    #[serde(rename = "pexp_desc")]
    desc: ExpressionDesc,
    //    location: Location,
    //    location_stack: Vec<Location>,
    #[serde(rename = "pexp_attributes")]
    attributes: Vec<Attribute>,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub enum ExpressionDesc {
    PexpConstant(Constant),
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct Constant {
    //  pconst_desc : constant_desc;
    #[serde(rename = "pconst_desc")]
    desc: ConstantDesc,
    location: Location,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub enum ConstantDesc {
    PconstString(String, Location, Option<String>),
}

//#[derive(serde::Deserialize, Debug)]
pub type Structure = Vec<StructureItem>;

//#[derive(serde::Deserialize, Debug)]
pub type Signature = Vec<SignatureItem>;

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct SignatureItem {
    // psig
    #[serde(rename = "psig_desc")]
    #[serde(deserialize_with = "deserialize::deserialize_signature_item")]
    desc: SignatureItemDesc,  
//    location: Location,
}

#[derive(serde::Deserialize, Debug)]
pub enum RecFlag {
    Recursive,
    NonRecursive,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub enum Payload {
    PStr(Structure),
    PSig(Signature),
    PTyp(CoreType),
    //    PPat(Pattern, Option<Expression>),
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct Attribute {
    name: LocString,
    payload: Payload,
//    location: Location,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct ModuleExpr {
    // pmod_
    #[serde(rename = "pmod_desc")]
    desc: ModuleExprDesc,
    //    location: Location,
    #[serde(rename = "pmod_attributes")]
    attributes: Vec<Attribute>,
}

#[derive(serde::Deserialize, Debug)]
pub enum ModuleExprDesc {
    PmodIdent,
    PmodStructure,
    PmodFunctor,
    PmodApply,
    PmodApplyUnit,
    PmodConstraint,
    PmodUnpack,
    PmodExtension,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct OpenInfos {
    expr: String,
    //    override_flag: OverrideFlag,
//    location: Location,
    attributes: Vec<Attribute>,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct OpenDescription {
    longident: LongIdent,
    open_infos: OpenInfos,
}

//#[derive(serde::Deserialize, Debug)]
#[derive(Debug)]
#[allow(dead_code)]
//#[serde(untagged)]
pub enum SignatureItemDesc {
    PsigType(RecFlag, Vec<TypeDeclaration>),
    PsigOpen(OpenDescription),
    PsigAttribute(Attribute),
    None,
}
