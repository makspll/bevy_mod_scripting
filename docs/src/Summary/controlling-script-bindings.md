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

Note these types are implemented for primitives, but if you want to interact with one of your `Reflect` implementing types, you will need to use one of `Ref<T>`, `Mut<T>` or `Val<T>` wrappers in place of `&T`, `&mut T` and `T` respectively.

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
    fn test_fn(_self: Ref<TestStruct>, mut arg1: usize) {}
}


pub fn main() {
    let mut app = App::new();
    register_test_fn(app.world_mut())
}
```

Note the documentation will automatically be picked up and stored for the purposes of reflection and documentation generation, including argument/return type specific docs.


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
| eq | an equality function, used for checking if two values are equal | ✅ | ❌ |
| lt | a less than function, used for checking if a value is less than another | ✅ | ❌ |
| iter | an iterator function, used for iterating over a value | ❌ | ✅ |
| display_ref | a display function, used for displaying a reference to a value | ❌ | ✅ |
| display_value | a display function, used for displaying a mutable reference to a value | ❌ | ✅ |

In this context `overridable` indicates whether language implementations will look for a specific function on your type before looking at the generic `ReflectReference` namespace. You can still remove the existing registration for these functions on the `ReflectReference` namespace if you want to replace them with your own implementation.

Note the `ReflectReference` namespace is special, in that functions defined on it, act like a fallback and hence apply to ALL references.


## Globals

By default, each type registered with the type registry, has the following set:
- a static reference in the global namespace, i.e.: `Vec3`, `Mat3`
- an entry in the `types` global type cache, i.e.: `types.Vec3`, `types.Mat3`

You can filter the types included by customising the `CoreScriptGlobalsPlugin`