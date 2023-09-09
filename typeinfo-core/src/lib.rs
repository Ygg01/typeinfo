use core::alloc::Layout;

/// Result of `typeinfo!`
struct Type {
    name: &'static str, // result of `type_name`
    inner: TypeInner,
    layout: Layout,
    generics: &'static [Generic],
    lifetimes: &'static [LifetimeTy],
}

struct DiscriminantTy {
    name: &'static str,
    // Zero based index of the enumeration
    discriminant: isize,
}

struct LifetimeTy {
    name: &'static str,
}

// impl Type {
//     /// Defer to `StructTy::field`, panic if not a struct
//     const fn field(name: &'static str) -> &'static StructField;
//     /// Defer to `EnumTy::variant`, panic if not an enum
//     const fn variant(name: &'static str) -> &'static EnumVariant;
// }

enum TypeInner {
    Struct(StructTy),
    Enum(EnumTy),
}

struct StructTy {
    fields: &'static [Field],
}
struct EnumTy {
    variants: &'static [EnumVariant],
}

struct EnumVariant {
    
    fields: &'static [Field],
    discriminant: DiscriminantTy,
    value: Option<isize>, // Value of C-style enums
}

struct Field {
    /// Field type
    field_type: Type,
    /// Field name if a named struct
    name: Option<&'static str>,
    /// Field count within the struct as defined
    field_index: usize,
    // offset in implementation
    offset: usize,
}

// impl Field {
//     // Helpers to get or set a field on the parent struct
//     // Not sure what this would do for enums
//     const fn getter(&self) -> (fn(&ParentTy) -> SelfTy) {

//     }
//     const fn setter(&self) -> fn(&mut ParentTy, SelfTy) {

//     }
// }

struct Generic {
    ty: Type,
    default: Option<Type>,
}

// impl StructTy { // similar for EnumTy with Variant
//     /// Get a field by name at compile time, compile_error if it doesn't exist
//     const fn field(name: &'static str) -> &'static Field;
// }
