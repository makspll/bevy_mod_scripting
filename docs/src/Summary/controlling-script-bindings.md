# Controlling Script Bindings

In this book we reffer to anything accessible by a script, which allows it to communicate with your Rust code a `binding` (which in previous versions was more generically referred to as a script API).

The "binding" here being used as in: binding `script` code to `rust` code.

## Dynamic Functions

Everything callable by scripts must first be registered in the dynamic function registry. Notably we do not make use of the normal bevy function registry to improve performance and usability. This means you cannot call just any function.

In order for a function to be callable by a script it must adhere to a few requirements:
- Each argument must implement `FromScript`.
- Each return type must implement `IntoScript`.
- Each argument must also implement `GetInnerTypeDependencies`
- Each return type must also implement `GetInnerTypeDependencies`

The into/from requirements allow us to convert these types to `ScriptValue`'s, and each supported scripting language can then marshall these into the script.

Note these types are implemented for primitives, but if you want to interact with one of your `Reflect` implementing types, you will need to use one of `Ref<T>`, `Mut<T>` or `Val<T>` wrappers in place of `&T`, `&mut T` and `T` respectively.

These wrappers enable us to safely interact with bevy, and claim any necessary mutex'es on `Resources`, `Components` or `Allocations`.

The `GetInnerTypeDependencies`, trait is simply a local trait alias for `GetTypeRegistration` with less strict type requirements. It allows us to register all the types necessary for the function calls, so that you don't have to register anything manually. If your type implements `GetTypeRegistration` you should not face any issues on this front.

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
```

This will allow you to call this function within lua like so:

```lua
hello_world("hi from lua!")
```

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

