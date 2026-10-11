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

A list is taken to be the fields in order, which is usually the more natural way to write tuple structs:

```lua
local my_tuple_struct = MyTupleStruct.new({ "hello" })
```

Types which are not structs, tuple structs, tuples or enums, can be constructed only if they implement `Default`, using `Type.new({})`.

### Defaults

Fields which are not provided are filled in, in the following order of priority:

1. the value from the type's `Default` implementation, if it has one. For enums, only if the default is the variant being constructed
2. the `Default` of the field's type, if it has one

If neither is available the construction fails, and the error names the missing field.
The generated Lua language server files mark exactly these fields as optional, see [Editor support](#editor-support).

### Naming and priority

`new` is a single function shared by all types, and is made available on every type's static reference.
A function with the same name registered on the type itself always takes priority, i.e. `Vec3.new(x, y, z)` is the generated `Vec3` constructor, not the shared one.
For such types use [`construct`](#construct), which works for every type.

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
local b = MyEnum.VariantB:new({ 5 })
local c = MyEnum.VariantC:new({})
```

The enum's `Default` only fills in fields if it is the variant being constructed.

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

The `construct` global function constructs types from a registration.
It works for every type, including those whose own `new` function shadows the shared one, and is useful if you only have the type as a value:

```lua
local MyStruct = types.MyStruct
local concrete_my_struct = construct(MyStruct, {
    my_field = "hello"
})

local variantA = construct(types.MyEnum, {
    variant = "VariantA",
    field = "hello"
})

local vec = construct(types.Vec3, { x = 1, y = 2, z = 3 })
```

Missing fields are filled in from defaults in the same way as for `new`.

## Editor support

The Lua language server files describe the table each constructor takes, so editors can complete and check the fields.
A field is optional exactly when a default can fill it in, as described in [Defaults](#defaults):

```rust,ignore
#[derive(Reflect)]
pub struct MyStruct {
    pub my_field: String,
    pub handle: MyHandle, // no `Default`
}
```

```lua
---@class (exact) MyStruct.Fields
---@field my_field? string
---@field handle MyHandle
---@field [any] 'never'

---@param payload MyStruct.Fields
---@return MyStruct
function MyStruct.new(payload) end
```

The language server then warns about missing required fields, fields of the wrong type, and unknown fields, i.e. a misspelled `my_feild`.
The last line rejects unknown fields: until the language server checks exact classes against tables passed to functions ([LuaLS/lua-language-server#1990](https://github.com/LuaLS/lua-language-server/issues/1990)), any undeclared field has to be of a type no value has, so the warning reads "Cannot assign `string` to `'never'`".

Tuple fields are declared under their integer keys, i.e. `[1]`, which a list such as `{ "hello" }` fills in, and also under their string keys, i.e. `["1"]`.
Both are type checked, but required fields are only recognised under their integer keys, so prefer lists or integer keys for tuple structs.

Each enum variant has a class of its own, i.e. `MyEnum.VariantA`, so `MyEnum.VariantA:new{` completes the fields of `VariantA` and returns a `MyEnum`.

The table of `MyEnum.new{ variant = "VariantA", ... }` is one of the variants' tables, each with its `variant`.
The language server checks the variant name and rejects fields no variant has, but it does not narrow the table down by the `variant` given,
so a field of another variant, or a missing required field, is not reported. Prefer `MyEnum.VariantA:new{...}` for the full checks.

Since the type of `construct`'s first argument is only known when the script runs, its table is not described.
