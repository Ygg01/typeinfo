#!feature[const_trait_impl]
use core::alloc::Layout;

/// Result of `typeinfo!`
#[derive(Clone, Debug)]
pub struct Type {
    pub name: &'static str,
    pub inner: TypeInner,
    pub layout: Layout,
    pub generics: &'static [Generic],
    pub lifetimes: &'static [LifetimeTy],
}

impl Default for Type {
    fn default() -> Self {
        Type {
            name: &"",
            inner: TypeInner::default(),
            layout: Layout::new::<Self>(),
            generics: &[],
            lifetimes: &[],
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct DiscriminantTy {
    pub name: &'static str,
    // Zero based index of the enumeration
    pub discriminant: isize,
}

#[derive(Clone, Debug, Default)]
pub struct LifetimeTy {
    pub name: &'static str,
}

// impl Type {
//     /// Defer to `StructTy::field`, panic if not a struct
//     const fn field(name: &'static str) -> &'static StructField;
//     /// Defer to `EnumTy::variant`, panic if not an enum
//     const fn variant(name: &'static str) -> &'static EnumVariant;
// }

#[derive(Clone, Debug, Default)]
pub enum TypeInner {
    Struct(StructTy),
    Enum(EnumTy),
    #[default]
    None,
}

#[derive(Clone, Debug, Default)]
pub struct StructTy {
    pub fields: &'static [Field],
}

#[derive(Clone, Debug, Default)]
pub struct EnumTy {
    pub variants: &'static [EnumVariant],
}

#[derive(Clone, Debug, Default)]
pub struct EnumVariant {
    pub fields: &'static [Field],
    pub discriminant: DiscriminantTy,
    pub value: Option<isize>, // Value of C-style enums
}

#[derive(Clone, Debug, Default)]
pub struct Field {
    /// Field type
    pub field_type: Type,
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

#[derive(Clone, Debug, Default)]
pub struct Generic {
    pub ty: Type,
    pub default: Option<Type>,
}

// impl StructTy { // similar for EnumTy with Variant
//     /// Get a field by name at compile time, compile_error if it doesn't exist
//     const fn field(name: &'static str) -> &'static Field;
// }

pub trait Reflect {}
