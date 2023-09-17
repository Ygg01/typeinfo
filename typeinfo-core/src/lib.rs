#!feature[const_trait_impl]
use core::alloc::Layout;

/// Result of `typeinfo!`
#[derive(Clone, Debug)]
pub struct TypeInfo {
    pub name: &'static str,
    pub inner: InnerTypeInfo,
    pub layout: Layout,
    pub generics: &'static [GenericInfo],
    pub lifetimes: &'static [LifetimeInfo],
}

#[derive(Clone, Debug)]
pub struct DiscriminantInfo {
    pub name: &'static str,
    // Zero based index of the enumeration
    pub discriminant: isize,
}

#[derive(Clone, Debug)]
pub struct LifetimeInfo {
    pub name: &'static str,
}

// impl Type {
//     /// Defer to `StructTy::field`, panic if not a struct
//     const fn field(name: &'static str) -> &'static StructField;
//     /// Defer to `EnumTy::variant`, panic if not an enum
//     const fn variant(name: &'static str) -> &'static EnumVariant;
// }

#[derive(Clone, Debug)]
pub enum InnerTypeInfo {
    // todo remove
    None,
    Struct(StructInfo),
    Enum(EnumInfo),
    Union(UnionEnumInfo),
}

#[derive(Clone, Debug)]
pub struct StructInfo {
    pub fields: &'static [FieldInfo],
}

#[derive(Clone, Debug)]
pub struct EnumInfo {
    pub variants: &'static [EnumVariantInfo],
}

#[derive(Clone, Debug)]
pub struct UnionEnumInfo {
    pub fields: &'static [FieldInfo],
}

#[derive(Clone, Debug)]
pub struct EnumVariantInfo {
    pub fields: &'static [FieldInfo],
    pub discriminant: DiscriminantInfo,
    pub value: Option<isize>, // Value of C-style enums
}

#[derive(Clone, Debug)]
pub struct FieldInfo {
    /// Field type
    pub field_type: TypeInfo,
    /// Field name if a named struct
    pub name: Option<&'static str>,
    /// Field count within the struct as defined
    pub field_index: usize,
    // offset in implementation
    pub offset: usize,
}

// impl Field {
//     // Helpers to get or set a field on the parent struct
//     // Not sure what this would do for enums
//     const fn getter(&self) -> (fn(&ParentTy) -> SelfTy) {

//     }
//     const fn setter(&self) -> fn(&mut ParentTy, SelfTy) {

//     }
// }

#[derive(Clone, Debug)]
pub struct GenericInfo {
    pub ty: TypeInfo,
    pub default: Option<TypeInfo>,
}

// impl StructTy { // similar for EnumTy with Variant
//     /// Get a field by name at compile time, compile_error if it doesn't exist
//     const fn field(name: &'static str) -> &'static Field;
// }

pub trait Reflect {}
