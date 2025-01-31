#!feature[const_trait_impl]

const fn const_bytes_equal(lhs: &[u8], rhs: &[u8]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }
    let mut i = 0;
    while i < lhs.len() {
        if lhs[i] != rhs[i] {
            return false;
        }
        i += 1;
    }
    true
}

const fn const_str_equal(lhs: &str, rhs: &str) -> bool {
    const_bytes_equal(lhs.as_bytes(), rhs.as_bytes())
}

/// Result of `typeinfo!`
#[derive(Clone, Debug)]
pub enum TypeInfo {
    UnitType,
    Struct(StructInfo),
    Enum(EnumInfo),
    Union(UnionEnumInfo),
    Function(FunctionInfo),
    Slice(SliceInfo),
    Array(ArrayInfo),
    Tuple(TupleInfo),
}

#[derive(Clone, Debug)]
pub struct DiscriminantInfo {
    pub name: &'static str,
    // Zero based index of the enumeration
    pub discriminant: isize,
}

#[derive(Clone, Copy, Debug)]
pub struct LifetimeInfo {
    pub name: &'static str,
}

#[derive(Clone, Debug)]
pub struct StructInfo {
    pub name: &'static str,
    pub lifetimes: &'static [LifetimeInfo],
    pub fields: &'static [FieldInfo],
}

impl StructInfo {
    pub const fn fields(&self, field_name: &'static str) -> Option<&FieldInfo> {
        let mut i = 0;

        while i < self.fields.len() {
            if let Some(field) = self.fields[i].name {
                if const_str_equal(field, field_name) {
                    return Some(&self.fields[i]);
                }
            }
            i += 1;
        }
        None
    }
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
pub struct FunctionInfo {
    pub fields: &'static [FieldInfo],
}

#[derive(Clone, Debug)]
pub struct SliceInfo {
    pub fields: &'static [FieldInfo],
}

#[derive(Clone, Debug)]
pub struct ArrayInfo {
    pub fields: &'static [FieldInfo],
}
#[derive(Clone, Debug)]
pub struct TupleInfo {
    pub fields: &'static [FieldInfo],
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

#[derive(Clone, Debug)]
pub struct GenericInfo {
    pub ty: TypeInfo,
}

pub trait Reflect {}
