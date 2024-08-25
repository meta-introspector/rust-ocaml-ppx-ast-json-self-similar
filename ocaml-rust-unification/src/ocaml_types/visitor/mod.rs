use serde::de::Visitor;
use std::fmt;
use serde::de::SeqAccess;

use crate::ocaml_types::CoreTypeDesc;
use crate::ocaml_types::CoreType;
use crate::ocaml_types::ConstructorArguments;
use crate::ocaml_types::RecFlag;
use crate::ocaml_types::TypeDeclaration;
use crate::ocaml_types::SignatureItemDesc;
use crate::ocaml_types::TypeKind;
use crate::ocaml_types::LabelDeclaration;
use crate::ocaml_types::ConstructorDeclaration;

pub struct CoreTypeDescVisitor;
pub struct ConstructorArgumentsVisitor;
pub struct TypeKindVisitor;
pub struct SignatureItemDescVisitor;
    
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
