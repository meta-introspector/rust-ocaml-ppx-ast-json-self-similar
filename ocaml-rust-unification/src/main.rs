use std::env;
//use /std::fmt;
use serde::de::Visitor;
use serde::de::{
    //Visitor,
    SeqAccess,
    //MapAccess
};
use serde::{
//    de,
//    Deserialize, //Serialize,
    Deserializer,
};
use std::fmt;
//use std::marker::PhantomData;

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
#[allow(dead_code)]
enum CoreTypeDesc {
    // Ptype_constr
    PtypConstr(LongIdent, Vec<CoreType>),
    // ... other variants ...
    None
}
struct CoreTypeDescVisitor;

impl<'de> Visitor<'de> for CoreTypeDescVisitor {
    type Value = CoreTypeDesc;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expecting a core type desc")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let v = seq.next_element::<serde_json::Value>();
	match v {
	    Ok(Some(a)) => {
		println!("CoreTypeDesc 1 {:?}",a);
		match a.as_str() {
		    Some("Ptyp_constr") => {
			println!("constr type {:?}",a);
			//| Ptyp_constr of Longident.t loc * core_type list
			let _ident:  String = seq.next_element()?.unwrap();
			let _core_type_list = seq.next_element::<Vec<CoreType>>()?.unwrap_or(Vec::new());
			return Ok(CoreTypeDesc::None);
		    },
		    _ => {
			println!("TEST OTHER {:?}",a);
			return Ok(CoreTypeDesc::None);
		    }
		}
	    },
	    Ok(None) => {
		println!("CoreTypeDesc seq none");
		return Ok(CoreTypeDesc::None);
	    },
	    Err(e) =>{
		println!("err seq 1 {:?}",e);
		return Ok(CoreTypeDesc::None);
	    }
	}
    }
}


#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
struct LocString {
    //loc2: {
    txt2: String
}

fn deserialize_core_type_desc<'de, D>(deserializer: D) -> Result<CoreTypeDesc, D::Error>
where
    D: Deserializer<'de>,
{
    let ct = deserializer.deserialize_seq(CoreTypeDescVisitor {});
    println!("core type{:?}",ct);
    Ok(CoreTypeDesc::None)
}

#[derive(serde::Deserialize, Debug)]
    #[allow(dead_code)]
struct CoreType {
    #[serde(rename = "ptyp_desc")]
    #[serde(deserialize_with = "deserialize_core_type_desc")]
    desc: CoreTypeDesc, // ptyp_
    //    location: Location,
    //    location_stack: Vec<Location>,
    #[serde(rename = "ptyp_attributes")]
    attributes: Vec<Attribute>,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
enum ConstructorArguments {
    Tuple(Vec<CoreType>),          // Tuple style constructor arguments (Pcstr_tuple)
    Record(Vec<LabelDeclaration>), // Record style constructor arguments (Pcstr_record)
    None
}

struct ConstructorArgumentsVisitor;

impl<'de> Visitor<'de> for ConstructorArgumentsVisitor {
    type Value = ConstructorArguments;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expecting a ConstructorArguments")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let v = seq.next_element::<serde_json::Value>();
	match v {
	    Ok(Some(a)) => {
		println!("Constructor Arguments {:?}",a);
		match a.as_str() {
		    Some("Psig_type") => {
			println!("psig type {:?}",a);
			// // | Psig_type of rec_flag * type_declaration list
			//let _rec_flag = seq.next_element::<serde_json::Value>();
			//			let _type_declaration_list = seq.next_element::<serde_json::Value>();
			let _rec_flag_vector: Vec<RecFlag> = seq.next_element()?.unwrap();
			//let _rec_flag = seq.next_element::<RecFlag>()?.unwrap_or(RecFlag::NonRecursive);
			let _type_declaration_list = seq.next_element::<Vec<TypeDeclaration>>()?.unwrap_or(Vec::new());

			return Ok(ConstructorArguments::None);
			//return Ok(decode_psig_type(seq));
		    },
		    _ => {
			println!("TEST OTHER {:?}",a);
			return Ok(ConstructorArguments::None);
		    }
		}
	    },
	    Ok(None) => {
		println!("TEST seq none");
		return Ok(ConstructorArguments::None);
	    },
	    Err(e) =>{
		println!("err seq 1 {:?}",e);
		return Ok(ConstructorArguments::None);
	    }
	}
    }
}

fn deserialize_constructor_arguments<'de, D>(deserializer: D) -> Result<ConstructorArguments, D::Error>
where
    D: Deserializer<'de>,
{
    let ct = deserializer.deserialize_seq(ConstructorArgumentsVisitor {});
    println!("core type{:?}",ct);
    Ok(ConstructorArguments::None)
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
struct ConstructorDeclaration {
    #[serde(rename = "pcd_name")]
    name: LocString,
    #[serde(rename = "pcd_vars")]
    vars: Vec<String>,
    
    #[serde(rename = "pcd_args")]
    #[serde(deserialize_with = "deserialize_constructor_arguments")]
    args: ConstructorArguments,
    #[serde(rename = "pcd_res")]
    res: Option<CoreType>,   
    //    location: Location,
    #[serde(rename = "pcd_attributes")]
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
#[allow(dead_code)]
enum TypeKind {
    Abstract,                             //Ptype_abstract,
    Variant(Vec<ConstructorDeclaration>), // Ptype_v
    Record(Vec<LabelDeclaration>),        //  (** Invariant: non-empty list *) Ptype_record
    Open,                                 //Ptype_open
    None
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
#[allow(dead_code)]
struct TypeDeclaration {
    #[serde(rename = "ptype_name")]
    name: LocString,
    #[serde(rename = "ptype_params")]
    params: Vec<(String, Variance, Injectivity)>,
    #[serde(rename = "ptype_cstrs")]
    constraints: Vec<(String, String, Location)>,

    
    #[serde(rename = "ptype_kind")]
    #[serde(deserialize_with = "deserialize_ptype_kind")]
    kind: TypeKind,

///    #[serde(rename = "ptype_private")]
    //    private: PrivateFlag,
    #[serde(rename = "ptype_manifest")]
    manifest: Option<String>,
    #[serde(rename = "ptype_attributes")]
    attributes: Vec<String>,
    //#[serde(rename = "ptype_loc")]
    //location: Location,
}

#[derive(serde::Deserialize, Debug)]
enum MutableFlag {
    Immutable,
    Mmutable,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
struct LabelDeclaration {
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
struct StructureItem {
    //     pstr_desc
    #[serde(rename = "pstr_desc")]
    desc: StructureItemDesc,
    //location: Location,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
enum StructureItemDesc {
    PstrEval(Expression, Vec<Attribute>),
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
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
#[allow(dead_code)]
enum ExpressionDesc {
    PexpConstant(Constant),
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
struct Constant {
    //  pconst_desc : constant_desc;
    #[serde(rename = "pconst_desc")]
    desc: ConstantDesc,
    location: Location,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
enum ConstantDesc {
    PconstString(String, Location, Option<String>),
}

//#[derive(serde::Deserialize, Debug)]
type Structure = Vec<StructureItem>;

//#[derive(serde::Deserialize, Debug)]
type Signature = Vec<SignatureItem>;

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
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
#[allow(dead_code)]
enum Payload {
    PStr(Structure),
    PSig(Signature),
    PTyp(CoreType),
    //    PPat(Pattern, Option<Expression>),
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
struct Attribute {
    name: LocString,
    payload: Payload,
//    location: Location,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
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
#[allow(dead_code)]
struct OpenInfos {
    expr: String,
    //    override_flag: OverrideFlag,
//    location: Location,
    attributes: Vec<Attribute>,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
struct OpenDescription {
    longident: LongIdent,
    open_infos: OpenInfos,
}

//#[derive(serde::Deserialize, Debug)]
#[derive(Debug)]
#[allow(dead_code)]
//#[serde(untagged)]
enum SignatureItemDesc {
    PsigType(RecFlag, Vec<TypeDeclaration>),
    PsigOpen(OpenDescription),
    PsigAttribute(Attribute),
    None,
}



///////////////// implementation
fn decode_psig_type<A>(mut _seq: A) -> SignatureItemDesc {
    SignatureItemDesc::None
}

fn decode_psig_open<A>(mut _seq: A) -> SignatureItemDesc {
    //(mut seq: A) -> Result(SignatureItemDesc,A::Error) {
    SignatureItemDesc::None
}

fn decode_psig_attribute<A>(mut _seq: A) -> SignatureItemDesc {
    //(mut seq: A) -> Result(SignatureItemDesc,A::Error) {
    SignatureItemDesc::None
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
		println!("Signature Item Desc 1 {:?}",a);
		match a.as_str() {
		    Some("Psig_type") => {
			println!("psig type {:?}",a);
			// // | Psig_type of rec_flag * type_declaration list
			//let _rec_flag = seq.next_element::<serde_json::Value>();
			//			let _type_declaration_list = seq.next_element::<serde_json::Value>();
			let _rec_flag_vector: Vec<RecFlag> = seq.next_element()?.unwrap();
			//let _rec_flag = seq.next_element::<RecFlag>()?.unwrap_or(RecFlag::NonRecursive);
			let _type_declaration_list = seq.next_element::<Vec<TypeDeclaration>>()?.unwrap_or(Vec::new());

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
			return Ok(SignatureItemDesc::None);
		    }
		}
	    },
	    Ok(None) => {
		println!("TEST seq none");
		return Ok(SignatureItemDesc::None);
	    },
	    Err(e) =>{
		println!("err seq 1 {:?}",e);
		return Ok(SignatureItemDesc::None);
	    }
	}
    }
}

fn deserialize_ptype_kind<'de, D>(deserializer: D) -> Result<TypeKind, D::Error>
where
    D: Deserializer<'de>,
{
    let foo = deserializer.deserialize_seq(TypeKindVisitor {});
    println!("type kind {:?}",foo);
    Ok(TypeKind::None)
}

fn deserialize_signature_item<'de, D>(deserializer: D) -> Result<SignatureItemDesc, D::Error>
where
    D: Deserializer<'de>,
{
    let foo = deserializer.deserialize_seq(SignatureItemDescVisitor {});
    println!("TEST {:?}",foo);
    Ok(SignatureItemDesc::None)
}

struct TypeKindVisitor;
impl<'de> Visitor<'de> for TypeKindVisitor {
    type Value = TypeKind;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expecting a type kind")
    }
    
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let v = seq.next_element::<serde_json::Value>();
	match v {
	    Ok(Some(a)) => {
		println!("Label Declaration 1 {:?}",a);
		match a.as_str() {
  // 		    and type_kind =
  // | Ptype_abstract
  // | Ptype_variant of constructor_declaration list
  // | Ptype_record of label_declaration list  (** Invariant: non-empty list *)
  // | Ptype_open

		    Some("Ptype_record") => {
			println!("psig type {:?}",a);
			let _labels: Vec<LabelDeclaration> = seq.next_element()?.unwrap();
			// // | Psig_type of rec_flag * type_declaration list
			//let _rec_flag = seq.next_element::<serde_json::Value>();
			//			let _type_declaration_list = seq.next_element::<serde_json::Value>();
			// let _rec_flag_vector: Vec<RecFlag> = seq.next_element()?.unwrap();
			// //let _rec_flag = seq.next_element::<RecFlag>()?.unwrap_or(RecFlag::NonRecursive);
			// let _type_declaration_list = seq.next_element::<Vec<TypeDeclaration>>()?.unwrap_or(Vec::new());
			// return Ok(decode_psig_type(seq));
			return Ok(TypeKind::None);
		    },

		    Some("Ptype_variant") => {
			println!("typevariant {:?}",a);
			let _constructors: Vec<ConstructorDeclaration> = seq.next_element()?.unwrap();
			return Ok(TypeKind::None);
		    },

		    // | Ptype_variant of constructor_declaration list
		    // | Ptype_abstract
		    // | Ptype_open

		    _ => {
			println!("TEST OTHER {:?}",a);
			return Ok(TypeKind::None);
		    }
		}
	    },
	    Ok(None) => {
		println!("TEST seq none");
		return Ok(TypeKind::None);
	    },
	    Err(e) =>{
		println!("err seq 1 {:?}",e);
		return Ok(TypeKind::None);
	    }
	}
    }
}

fn main() {
    println!("Hello, world!");

    for argument in env::args_os().skip(1) {
        let json_file_path = argument;
        let s = json_file_path.into_string().unwrap();
        println!("read = {:?}", s);
        let deserialized: Signature = serde_json::from_reader(File::open(s).unwrap()).unwrap();
        println!("deserialized = {:?}", deserialized);
    }
}
