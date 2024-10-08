//pub mod deserialize;
use serde::Deserializer;
use crate::ocaml_types::CoreTypeDesc;
use crate::ocaml_types::ConstructorArguments;
use crate::ocaml_types::TypeKind;
use crate::ocaml_types::SignatureItemDesc;
use crate::ocaml_types::visitor::CoreTypeDescVisitor;
use crate::ocaml_types::visitor::ConstructorArgumentsVisitor;
use crate::ocaml_types::visitor::TypeKindVisitor;
use crate::ocaml_types::visitor::SignatureItemDescVisitor;
pub fn deserialize_core_type_desc<'de, D>(deserializer: D) -> Result<CoreTypeDesc, D::Error>
where
    D: Deserializer<'de>,
{
    let ct = deserializer.deserialize_seq(CoreTypeDescVisitor {});
    println!("core type{:?}",ct);
    Ok(CoreTypeDesc::None)
}

pub fn deserialize_constructor_arguments<'de, D>(deserializer: D) -> Result<ConstructorArguments, D::Error>
where
    D: Deserializer<'de>,
{
    let ct = deserializer.deserialize_seq(ConstructorArgumentsVisitor {});
    println!("core type{:?}",ct);
    Ok(ConstructorArguments::None)
}

pub fn deserialize_ptype_kind<'de, D>(deserializer: D) -> Result<TypeKind, D::Error>
where
    D: Deserializer<'de>,
{
    let foo = deserializer.deserialize_seq(TypeKindVisitor {});
    println!("type kind {:?}",foo);
    Ok(TypeKind::None)
}

pub fn deserialize_signature_item<'de, D>(deserializer: D) -> Result<SignatureItemDesc, D::Error>
where
    D: Deserializer<'de>,
{
    let foo = deserializer.deserialize_seq(SignatureItemDescVisitor {});
    println!("TEST {:?}",foo);
    Ok(SignatureItemDesc::None)
}
