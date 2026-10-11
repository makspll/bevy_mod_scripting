# Constructing Arbitrary Types

When interfacing with bevy, we do this via reflection.
While the generated bindings do not cover constructors for every single type that bevy or other libraries provide, reflection allows us to construct some (not all types implement `FromReflect`) types from dynamic structs.

BMS exposes this ability to all script writers via the `new` function available on every type, as well as the more verbose `construct` global function.

## `Type.new`

Every type available as a global has a `new` function, taking a table of the fields to set:

```rust,ignore
pub struct MyStruct {
    pub my_field: String,
    pub other_field: usize,
}
```

```lua
local my_struct = MyStruct.new({
    my_field = "hello",
    other_field = 5,
})
```

```rust,ignore
pub struct MyTupleStruct(pub String);
```

Tuple structs and tuples are constructed using the index of the field as the key, the keys follow the indexing convention of the language, i.e. in Lua the first field is `"1"`:

```lua
local my_tuple_struct = MyTupleStruct.new({
    ["1"] = "hello",
})
```

Types which are not structs, tuple structs, tuples or enums, can be constructed only if they implement `Default`, using `Type.new({})`.

### Defaults

Fields which are not provided are filled in, in the following order of priority:

1. the value from the type's `Default` implementation, if it has one (`use_type_default`)
2. the `Default` of the field's type, if it has one (`use_field_default`)

If neither is available the construction fails, and the error names the missing field.

Both can be turned off using the optional arguments of `new`, both are `true` by default:

```lua
-- the type's own `Default` is not consulted, only the defaults of the individual fields
local a = MyStruct.new({ my_field = "hello" }, false, true)

-- every field must be given
local b = MyStruct.new({ my_field = "hello", other_field = 5 }, false, false)
```

### Naming and priority

`new` is a single function shared by all types, and is made available on every type's static reference.
A function with the same name registered on the type itself always takes priority, i.e. `Vec3.new(x, y, z)` is the generated `Vec3` constructor, not the shared one.

In Rhai `new` is a reserved word, and is available as `new_`, as for all other functions named like a keyword.

The name of the function is configurable via the `ConstructorsPlugin`:

```rust,ignore
app.add_plugins(BMSPlugin.set(ConstructorsPlugin {
    constructor_name: "create".into(),
}));
```

The function is registered under the `StaticTypeRef` type, which is where functions available on all types are registered.

## Enums

The following enum:

```rust,ignore
pub enum MyEnum {
    VariantA {
        field: String
    },
    VariantB(usize),
    VariantC,
}
```

has its variants available as `MyEnum.VariantA`, `MyEnum.VariantB` and `MyEnum.VariantC`, which can be used to construct values of the variant using `:new`:

```lua
local a = MyEnum.VariantA:new({ field = "hello" })
local b = MyEnum.VariantB:new({ ["1"] = 5 })
local c = MyEnum.VariantC:new({})
```

`:new` takes the same optional `use_type_default` and `use_field_default` arguments as `Type.new`.
The type's `Default` is only used if it is the variant being constructed.

You can also pick the variant by name using `MyEnum.new({ variant = "VariantA", field = "hello" })`.

### Discriminating variants

The variant a value is in can be obtained with `:variant()`, and compared with the variants of the enum:

```lua
if my_enum:variant() == MyEnum.VariantA then
    print(my_enum.field)
end
```

This is also useful with enums that carry no data, such as keys:

```lua
if event.key_code:variant() == KeyCode.KeyA then
    print("A was pressed")
end
```

`:variant()` returns `nil` if the value is not an enum. If you need the name of the variant, `:variant_name()` is also available on both values and variants.

### Variants named like functions

Functions on the type take priority over variants, so a variant named like a function cannot be accessed with `MyEnum.Name`.
Use `world.get_enum_variant` to get at it in that case:

```lua
local variant = world.get_enum_variant(types.MyEnum, "Name")
```

## `construct`

The `construct` global function constructs types from a registration, and is useful if you only have the type as a value:

```lua
local MyStruct = types.MyStruct
local concrete_my_struct = construct(MyStruct, {
    my_field = "hello"
})

local variantA = construct(types.MyEnum, {
    variant = "VariantA",
    field = "hello"
})
```

Unlike `new`, `construct` does not use the `Default` implementation of the type itself, only the defaults of the fields.
