#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct Root {
    items: Vec<ModContainer>,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct ModContainer {
    Mod : Option<Mod>,
    Enum : Option<Enum>,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct Mod {
    ident: String
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct Struct {
    ident: String
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct Enum {
    ident: String,
    //attrs: Vec<EnumAttrs>
    variants: EnumVariants
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct EnumVariantObj {
    ident: Option<String>
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct EnumVariants {
    inner: Vec<Vec<EnumVariantObj>>
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct Type {
}
