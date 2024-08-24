use std::env;
//use /std::fmt;
use serde::de::Visitor;
use serde::de::{
    //Visitor,
    SeqAccess,
    //MapAccess
};
use serde::{
    de,
    Deserialize, //Serialize,
    Deserializer,
};
use std::fmt;
use std::marker::PhantomData;

use std::fs::File;
//use serde::de::Visitor;
//use serde_json::Value;
use serde_json;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, serde::Serialize)]
enum LongIdent {
    Lident(String),
    Ldot(Box<LongIdent>, String),
    Lapply(Box<LongIdent>, Box<LongIdent>),
}

#[derive(serde::Deserialize, Debug)]
enum CoreTypeDesc {
    PtypConstr(LongIdent, Vec<CoreType>),
    // ... other variants ...
}

#[derive(serde::Deserialize, Debug)]
struct CoreType {
    #[serde(rename = "ptyp_desc")]
    desc: CoreTypeDesc, // ptyp_
    //    location: Location,
    //    location_stack: Vec<Location>,
    #[serde(rename = "ptyp_attributes")]
    attributes: Vec<Attribute>,
}

#[derive(serde::Deserialize, Debug)]
enum ConstructorArguments {
    Tuple(Vec<CoreType>),          // Tuple style constructor arguments (Pcstr_tuple)
    Record(Vec<LabelDeclaration>), // Record style constructor arguments (Pcstr_record)
}

#[derive(serde::Deserialize, Debug)]
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

#[derive(serde::Deserialize, Debug)]
enum TypeKind {
    Abstract,                             //Ptype_abstract,
    Variant(Vec<ConstructorDeclaration>), // Ptype_v
    Record(Vec<LabelDeclaration>),        //  (** Invariant: non-empty list *) Ptype_record
    Open,                                 //Ptype_open
}

#[derive(serde::Deserialize, Debug)]
struct Location {}

#[derive(serde::Deserialize, Debug)]
enum Variance {
    Covariant,
    Contravariant,
    NoVariance,
}

#[derive(serde::Deserialize, Debug)]
enum Injectivity {
    Injective,
    NoInjectivity,
}

#[derive(serde::Deserialize, Debug)]
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

#[derive(serde::Deserialize, Debug)]
enum MutableFlag {
    Immutable,
    Mmutable,
}

#[derive(serde::Deserialize, Debug)]
struct LabelDeclaration {
    name: String,
    mutable: MutableFlag,
    type_: CoreType,
    location: Location,
    attributes: Vec<Attribute>,
}

#[derive(serde::Deserialize, Debug)]
struct StructureItem {
    //     pstr_desc
    #[serde(rename = "pstr_desc")]
    desc: StructureItemDesc,
    //location: Location,
}

#[derive(serde::Deserialize, Debug)]
enum StructureItemDesc {
    PstrEval(Expression, Vec<Attribute>),
}

#[derive(serde::Deserialize, Debug)]
struct Expression {
    //     pexp_desc: expression_desc;
    #[serde(rename = "pexp_desc")]
    desc: ExpressionDesc,
    //    location: Location,
    //    location_stack: Vec<Location>,
    #[serde(rename = "pexp_attributes")]
    attributes: Vec<Attribute>,
}

#[derive(serde::Deserialize, Debug)]
enum ExpressionDesc {
    PexpConstant(Constant),
}

#[derive(serde::Deserialize, Debug)]
struct Constant {
    //  pconst_desc : constant_desc;
    #[serde(rename = "pconst_desc")]
    desc: ConstantDesc,
    location: Location,
}

#[derive(serde::Deserialize, Debug)]
enum ConstantDesc {
    PconstString(String, Location, Option<String>),
}

//#[derive(serde::Deserialize, Debug)]
type Structure = Vec<StructureItem>;

//#[derive(serde::Deserialize, Debug)]
type Signature = Vec<SignatureItem>;

#[derive(serde::Deserialize, Debug)]
struct SignatureItem {
    // psig
    #[serde(rename = "psig_desc")]
    #[serde(deserialize_with = "deserialize_signature_item")]
    desc: SignatureItemDesc,
//    location: Location,
}

#[derive(serde::Deserialize, Debug)]
enum RecFlag {
    Recursive,
    NonRecursive,
}

#[derive(serde::Deserialize, Debug)]
enum Payload {
    PStr(Structure),
    PSig(Signature),
    PTyp(CoreType),
    //    PPat(Pattern, Option<Expression>),
}

#[derive(serde::Deserialize, Debug)]
struct Attribute {
    name: String,
    payload: Payload,
//    location: Location,
}

#[derive(serde::Deserialize, Debug)]
struct ModuleExpr {
    // pmod_
    #[serde(rename = "pmod_desc")]
    desc: ModuleExprDesc,
    //    location: Location,
    #[serde(rename = "pmod_attributes")]
    attributes: Vec<Attribute>,
}

#[derive(serde::Deserialize, Debug)]
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

#[derive(serde::Deserialize, Debug)]
struct OpenInfos {
    expr: String,
    //    override_flag: OverrideFlag,
//    location: Location,
    attributes: Vec<Attribute>,
}

#[derive(serde::Deserialize, Debug)]
struct OpenDescription {
    longident: LongIdent,
    open_infos: OpenInfos,
}

//#[derive(serde::Deserialize, Debug)]
#[derive(Debug)]
//#[serde(untagged)]
enum SignatureItemDesc {
    PsigType(RecFlag, Vec<TypeDeclaration>),
    PsigOpen(OpenDescription),
    PsigAttribute(Attribute),
    PsigNone,
}



///////////////// implementation
fn decode_psig_type<A>(mut _seq: A) -> SignatureItemDesc {
    SignatureItemDesc::PsigNone
}

fn decode_psig_open<A>(mut _seq: A) -> SignatureItemDesc {
    //(mut seq: A) -> Result(SignatureItemDesc,A::Error) {
    SignatureItemDesc::PsigNone
}

fn decode_psig_attribute<A>(mut _seq: A) -> SignatureItemDesc {
    //(mut seq: A) -> Result(SignatureItemDesc,A::Error) {
    SignatureItemDesc::PsigNone
}

struct SignatureItemDescVisitor;

impl<'de> Visitor<'de> for SignatureItemDescVisitor {
    type Value = SignatureItemDesc;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expecting a signature item descriptor")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let v = seq.next_element::<serde_json::Value>();
	match v {
	    Ok(Some(a)) => {
		println!("TEST seq 1 {:?}",a);
		match a.as_str() {
		    Some("Psig_type") => {
			println!("psig type {:?}",a);
			// // | Psig_type of rec_flag * type_declaration list
			let _rec_flag = seq.next_element::<serde_json::Value>();
			let _type_declaration_list = seq.next_element::<serde_json::Value>();
			return Ok(decode_psig_type(seq));
		    },
		    Some("Psig_open") => {
			println!("Psig open {:?}",a);
			// //| Psig_open of open_description  (** [open X] *)
			let _open_description = seq.next_element::<serde_json::Value>();
			return Ok(decode_psig_open(seq));
		    },		    
		    Some("Psig_attribute") => {
			println!("psig attribute {:?}",a);
			// //| Psig_attribute of attribute  (** [[\@\@\@id]] *)
			let _attribute = seq.next_element::<serde_json::Value>();
			return Ok(decode_psig_attribute(seq));
		    }
		    _ => {
			println!("TEST OTHER {:?}",a);
			return Ok(SignatureItemDesc::PsigNone);
		    }
		}
	    },
	    Ok(None) => {
		println!("TEST seq none");
		return Ok(SignatureItemDesc::PsigNone);
	    },
	    Err(e) =>{
		println!("err seq 1 {:?}",e);
		return Ok(SignatureItemDesc::PsigNone);
	    }
	}
    }
}

fn deserialize_signature_item<'de, D>(deserializer: D) -> Result<SignatureItemDesc, D::Error>
where
    D: Deserializer<'de>,
{
    let foo = deserializer.deserialize_seq(SignatureItemDescVisitor {});
    println!("TEST {:?}",foo);
    Ok(SignatureItemDesc::PsigNone)
}


fn main() {
    println!("Hello, world!");

    for argument in env::args_os().skip(1) {
        let json_file_path = argument;
        let s = json_file_path.into_string().unwrap();
        println!("read = {:?}", s);
        let deserialized: Signature = serde_json::from_reader(File::open(s).unwrap()).unwrap();
        println!("deserialized = {:?}", deserialized);

        //	let res = AstDeserializer::deserialize(&json_data);
    }
}
