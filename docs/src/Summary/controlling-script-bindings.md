# Controlling Script Bindings

In this book we refer to anything accessible by a script, which allows it to communicate with your Rust code a `binding` (which in previous versions was more generically referred to as a script API).

The "binding" here being used as in: binding `script` code to `rust` code.

## Namespaces

Namespaces are a way to group functions together, and are used to prevent naming conflicts. You can have multiple namespaces, and each namespace can have multiple functions.

Language implementations will also look for specific functions registered on your type first before looking at the generic `ReflectReference` namespace.

## Dynamic Functions

Everything callable by scripts must first be registered in the dynamic function registry. Notably we do not make use of the normal bevy function registry to improve performance and usability. This means you cannot call just any function.

In order for a function to be callable by a script it must adhere to a few requirements:
- Each argument must implement `FromScript`.
- Each return type must implement `IntoScript`.
- Each argument must also implement `GetTypeDependencies`
- Each return type must also implement `GetTypeDependencies`

The into/from requirements allow us to convert these types to `ScriptValue`'s, and each supported scripting language can then marshall these into the script.

Note these types are implemented for primitives, but if you want to interact with one of your `Reflect` implementing types, you will need to use one of `R<T>`, `M<T>` or `V<T>` wrappers in place of `&T`, `&mut T` and `T` respectively.

These wrappers enable us to safely interact with bevy, and claim any necessary mutex'es on `Resources`, `Components` or `Allocations`.

The `GetTypeDependencies`, trait is simply a local trait alias for `GetTypeRegistration` with less strict type requirements. It allows us to register all the types necessary for the function calls, so that you don't have to register anything manually. If your type implements `GetTypeRegistration` you should not face any issues on this front.

## Registering Script Functions

Registering functions can be done via the `NamespaceBuilder` like below:

```rust,ignore
    NamespaceBuilder::<ReflectReference>::new(&mut world)
        .register(
            "hello_world",
            |s: String| {
                println!(s)
            },
        );

    NamespaceBuilder::<GlobalNamespace>::new_unregistered(&mut world)
        .register(
            "hello_world2",
            |s: String| {
                println!(s)
            },
        );
```

This will allow you to call this function within lua like so:

```lua
some_type:hello_world("hi from method!");
hello_world2("hi from global!");
```

Note the `new_unregistered` call instead of `new`, this is because `GlobalNamespace` is not a `Reflect` type, and the `new` call also automatically registers the type in the reflection registry.

## Macros
The above is a bit tedious, so instead you can use the `script_bindings` macro, which applies to impl blocks like so:

```rust,ignore
#[script_bindings(name = "test_fn")]
impl TestStruct {
    /// My docs !!
    /// 
    /// Arguments:
    /// * `_self` - the first argument
    /// * `arg1` - the second argument
    /// Returns:
    /// * `return` - nothing
    fn test_fn(_self: R<TestStruct>, mut arg1: usize) {}
}


pub fn main() {
    let mut app = App::new();
    register_test_fn(app.world_mut())
}
```

Note the documentation will automatically be picked up and stored for the purposes of reflection and documentation generation, including argument/return type specific docs.

## Deriving FromScript and IntoScript

Generally all reflect types are wrapped in: `R<T>` `V<T>` or `M<T>` when writing script bindings. However some types may opt in to implement `FromScript` or `IntoScript` directly.

Doing this requires implementing a set of other traits which tell the runtime how to manage your type in conversion and function dispatch.

The most trivial way to allow your type to be used directly in bindings is to apply derive macros like so:

```rust
#[derive(Reflect, ScriptType)]
pub struct MyThing(usize);


#[script_bindings(remote)]
impl MyThing {
    // Can now be used directly without V<> wrapper, acts exactly like V<MyThing>
    pub fn test(thing: MyThing) -> MyThing {
        thing
    }
}
```

`ScriptType` implements `FromScript`, `IntoScript`, `ArgMeta`, `GetTypeDependencies` and `TypedThrough` at once, converting through `V<Self>`.

If you wish to customize one of the conversions, write that impl by hand and use `#[script_type(from = custom)]` or `#[script_type(into = custom)]`. This skips the generated impl and documents the type as opaque in generated docs. See [the custom conversions example](https://github.com/makspll/bevy_mod_scripting/blob/main/examples/custom_conversions.rs) for more information.

## Context Arguments

Each script function call always receives an additional context argument: `FunctionCallContext`.
You can opt-in to receive this argument in your own function definitions by adding it as the first argument.

The context contains requests from the caller to your function, such as "I am calling you from a 1-indexed array system, please convert the index first", This argument is only relevant if you're targeting multiple languages.

It also allows you to retrieve the world via `FunctionCallContext::world()`.

You can use this as follows:

```rust,ignore
    NamespaceBuilder::<ReflectReference>::new(&mut world)
        .register(
            "hello_world",
            |ctx: FunctionCallContext, s: String| {
                let world = ctx.world()?;
                let should_use_0_indexing = ctx.convert_to_0_indexed;
                println!(should_use_0_indexing);
                println!(s)
                Ok(())
            },
        );
```

## Generic Arguments

Sometimes you might want to be generic over the type of argument you're accepting, you can do so by accepting `ScriptValue` arguments like so:

```rust,ignore
    NamespaceBuilder::<ReflectReference>::new(&mut world)
        .register(
            "is_integer",
            |s: ScriptValue| {
                match s {
                    ScriptValue::Integer(i) => true,
                    _ => false
                }
            },
        );
```

You can treat return values similarly.

## Fallible functions

Your script functions can return errors either by:
- Returning `Result<T: IntoScript, InteropError>`
- Returning `ScriptValue` and manually creating the `ScriptValue::Error(into_interop_erorr.into())` variant.

## Reserved Functions

There are a few reserved functions that you can override by registering them on a specific type:

| Function Name | Description | Overridable? | Has Default Implementation? |
|---------------|-------------| ------------ | --------------------------- |
| get | a getter function, used for indexing into a type | ❌ | ✅ |
| set | a setter function, used for setting a value on a type | ❌ | ✅ |
| sub | a subtraction function, used for subtracting two values | ✅ | ❌ |
| add | an addition function, used for adding two values | ✅ | ❌ |
| mul | a multiplication function, used for multiplying two values | ✅ | ❌ |
| div | a division function, used for dividing two values | ✅ | ❌ |
| rem | a remainder function, used for getting the remainder of two values | ✅ | ❌ |
| neg | a negation function, used for negating a value | ✅ | ❌ |
| pow | a power function, used for raising a value to a power | ✅ | ❌ | 
| rsub, rdiv, rrem, rpow | the reflected `sub`, `div`, `rem` and `pow`, called on the right operand when the left one is not a value, i.e. `1 - v` calls `rsub(v, 1)`. `add` and `mul` are called with their operands swapped instead | ✅ | ❌ |
| eq | an equality function, used for checking if two values are equal | ✅ | ❌ |
| lt | a less than function, used for checking if a value is less than another | ✅ | ❌ |
| iter | an iterator function, used for iterating over a value | ❌ | ✅ |
| display | a display function, used for pretty printing values | ❌\* | ✅ |
| debug | a display function, used for displaying the internals of a value | ❌\* | ✅ |

\* - You can register an instance of `ReflectDisplayWithTypeInfo` type data on `Reflect` implementing types to override their printing behavior

In this context `overridable` indicates whether language implementations will look for a specific function on your type before looking at the generic `ReflectReference` namespace. You can still remove the existing registration for these functions on the `ReflectReference` namespace if you want to replace them with your own implementation.

Note the `ReflectReference` namespace is special, in that functions defined on it, act like a fallback and hence apply to ALL references.

### Where functions can live

The type in the `impl` block you apply `script_bindings` to (or the one passed to `NamespaceBuilder`) decides where the functions are available:

| `impl` target | Available on | Receives |
|---------------|--------------|----------|
| `MyType` | `MyType` values and the `MyType` type, as imported from its namespace | whatever you declare |
| `ReflectReference` | every value, after `MyType`'s own functions | the value |
| `StaticTypeRef` | every type, after `MyType`'s own functions | the type, filled in automatically |
| `NamespaceMembers` | the namespace given by `namespace = "..."`, i.e. `import_namespace("my_game")` | whatever you declare |
| `GlobalNamespace` | the builtin globals, i.e. no prefix | whatever you declare |

When the same name exists in more than one place, the most specific one wins.
For `ReflectReference` and `StaticTypeRef` in a `script_bindings` block outside of the crate defining them, use the `remote` attribute.

### Functions available on every type

The `StaticTypeRef` namespace is the equivalent for types, i.e. `Vec3` in `Vec3.new(..)`.
Functions registered on it are available on every type, and receive the type as their first argument, which is filled in automatically:

```rust,ignore
NamespaceBuilder::<StaticTypeRef>::new(app.world_mut()).register(
    "describe",
    |ctxt: FunctionCallContext, type_ref: V<StaticTypeRef>| -> Result<String, InteropError> {
        Ok(type_ref.type_info(&ctxt.world()?)?.type_path().to_string())
    },
);
```

```lua
local Vec3 = import_namespace("math").Vec3
print(Vec3.describe()) -- "glam::Vec3"
```

A function registered on the type itself always takes priority over one on `StaticTypeRef`.
The `new` constructor is implemented this way, see [Constructing Arbitrary Types](../ScriptingReference/constructing-arbitrary-types.md).

#### Signatures which depend on the type

A shared function's signature can refer to the type it is bound to, using the placeholder types `BoundValue` (a value of the type), `BoundFields` (a table of the type's fields) and `BoundHandle` (a handle to an asset of the type).
Wherever the function is bound to a type, these are replaced with the type, so the documentation and the Lua language server files show, for example, `MyStruct.new(payload: MyStruct.Fields): MyStruct` rather than a generic table and reference, or `Image.load(path: string): Handle<Image>`.
Functions using `BoundHandle` are only documented on asset types, i.e. types registered with `register_asset_reflect`.
The same placeholders work for functions on `EnumVariantRef`, which are bound to each variant of each enum when the LAD file is built (as functions in the `Variant` namespace). Placeholders left unbound are documented as what they are at runtime, a `ReflectReference` and a table.

The enum variants of a type are also available on the type, i.e. `MyEnum.Variant`, after the functions.


## Namespace members

Each type registered with the type registry is an entry in the table `import_namespace` returns for its [namespace](../ScriptingReference/namespaces.md), i.e. `import_namespace("math").Vec3`.

Functions which don't belong to a type are members of a namespace, declared on `impl NamespaceMembers`:

```rust,ignore
#[script_bindings(namespace = "my_game")]
impl NamespaceMembers {
    fn spawn_enemy(health: f32) { /* ... */ }
}
```

```lua
local my_game = import_namespace("my_game")
my_game.spawn_enemy(5.0)
```

Values are registered per namespace with `AppScriptNamespaceMembers::register_value`, and are made each time the namespace is imported.

The only globals are the builtins: `import_namespace`, `export_namespace` and `register_callback`. `world` is in the `bms` namespace: `import_namespace("bms").world`.

Types are not globals, which keeps types with the same name from shadowing each other. Use the `filter` of `CoreScriptGlobalsPlugin` to choose which types scripts can look up by name.