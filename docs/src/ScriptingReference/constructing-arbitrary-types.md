# Constructing Arbitrary Types

When interfacing with bevy, we do this via reflection.
While the generated bindings do not cover constructors for every single type that bevy or other libraries provide, reflection allows us to construct some (not all types implement `FromReflect`) types from dynamic structs.

BMS exposes this ability to all script writers via the `construct` global function.


## Structs

The following struct:
```rust,ignore
pub struct MyStruct {
    pub my_field: String
}
```

can be constructed from lua like so:
```lua
local MyStruct = world.get_type_by_name("MyStruct")
local concrete_my_struct = construct(MyStruct, {
    my_field = "hello"
})
```

## Tuple Structs
The following tuple struct:
```rust,ignore

pub struct MyTupleStruct(pub String);
```

can be constructed like so:
```lua

local MyTupleStruct = world.get_type_by_name("MyTupleStruct")
local concrete_my_tuple_struct = construct(MyTupleStruct, {
    _1 = "hello"
})
```

## Enums
The following enum:
```rust,ignore
pub enum MyEnum {
    VariantA {
        field: String
    },
    VariantB
}
```

can be constructed like so:
```lua

local MyEnum = world.get_type_by_name("MyEnum")
local variantA = construct(MyEnum, {
    variant = "VariantA",
    field = "hello"
})
local variantB = construct(MyEnum, {
    variant = "VariantB"
})
```

When working with enums you can also figure out the variant at runtime using `variant_name`:

```lua
if my_enum:variant_name() == "VariantA" then
    print(my_enum.field)
end
```