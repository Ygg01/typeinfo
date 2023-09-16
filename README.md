# This is an experimental crate in adding compile time reflection in Rust via Macros don't use it for anything

## Dev Diary

* Sep 8th - Trevor gross had idea about [reflection types](https://rust-lang.zulipchat.com/#narrow/stream/122651-general/topic/Compile.20time.20reflection/near/389927146)

```rust
// Everything is non_exhaustive and public
// (not sure how to handle private fields...)

/// Result of `typeinfo!`
struct Type {
    name: &'static str, // result of `type_name`
    inner: TypeInner,
    layout: Layout,
    generics: &'static[Generic],
    lifetimes: &'static[Lifetime],
}

impl Type {
    /// Defer to `StructTy::field`, panic if not a struct
    const fn field(name: &'static str) -> &'static StructField;
    /// Defer to `EnumTy::variant`, panic if not an enum
    const fn variant(name: &'static str) -> &'static EnumVariant;
}

enum TypeInner { Struct(StructTy), Enum(EnumTy), ...}

struct StructTy { fields: &'static[Field] }
struct EnumTy { variants: &'static[EnumVariant] }

struct EnumVariant {
    fields: &'static[Field],
    discriminant: Discriminant,
    value: Option<isize> // Value of C-style enums
}

struct Field {
    type: Type,   // Field type
    name: Option<&'static str>,  // Field name if a named struct
    field_index: usize,  // Field count within the struct as defined
    offset: usize,  // offset in implementation
}

impl Field {
    // Helpers to get or set a field on the parent struct
    // Not sure what this would do for enums
    const fn getter(&self) -> (fn(&ParentTy) -> SelfTy);
    const fn setter(&self) -> fn(&mut ParentTy, SelfTy);
}

struct Generic {
    ty: Type,
    default: Option<Type>
}

impl StructTy { // similar for EnumTy with Variant
    /// Get a field by name at compile time, compile_error if it doesn't exist
    const fn field(name: &'static str) -> &'static Field;
}

// ...
```
* Sep 9th - Can't have `const fn` in `trait`, maybe use associated constants in traits
  per [fee1-dead](https://rust-lang.zulipchat.com/#narrow/stream/122651-general/topic/Compile.20time.20reflection/near/390039703)
* Sep 15th - [Why a feature vs a proc macro (Jiahao XU)](https://rust-lang.zulipchat.com/#narrow/stream/122651-general/topic/Compile.20time.20reflection/near/391031261) - essentially alternative approach and Drawbacks.



## Prior art:

- [A mirror for Rust](https://archive.is/pgWQo)
- [Frunk](https://github.com/lloydmeta/frunk)
- [Castaway](https://github.com/sagebind/castaway)
