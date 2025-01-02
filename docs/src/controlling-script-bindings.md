# Controlling Script Bindings

In this book we reffer to anything accessible by a script, which allows it to communicate with your Rust code a `binding` (which in previous versions was more generically referred to as a script API).

The "binding" here being used as in: binding `script` code to `rust` code.

## Function Registry

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



