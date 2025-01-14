# Evaluating Feasibility

In order for a language to work well with BMS it's necessary it supports the following features:
- [ ] Interoperability with Rust. If you can't call it from Rust easilly and there is no existing crate that can do it for you, it's a no-go.
- [ ] First class functions. Or at least the ability to call an arbitrary function with an arbitrary number of arguments from a script. Without this feature, you would need to separately generate code for the bevy bindings which is painful and goes against the grain of BMS.

## First Classs Functions

They don't necessarily have to be first class from the script POV, but they need to be first class from the POV of the host language. This means that the host language needs to be able to call a function with an arbitrary number of arguments. 

### Examples

Let's say your language supports a `Value` type which can be returned to the script. And it has a `Value::Function` variant. The type on the Rust side would look something like this:

```rust,ignore
pub enum Value {
    Function(Arc<Fn(&[Value]) -> Value>),
    // other variants
}
```

This is fine, and can be integrated with BMS. Since an Fn function can be a closure capturing a `DynamicScriptFunction`. If there is no support for `FnMut` closures though, you might face issues in the implementation. Iterators in `bevy_mod_scripting_functions` for example use `DynamicScriptFunctionMut` which cannot work with `Fn` closures.

Now let's imagine instead another language with a similar enum, supports this type instead:

```rust
pub enum Value {
    Function(Arc<dyn Function>),
    // other variants
}

pub trait Function {
    fn call(&self, args: Vec<Value>) -> Value;

    fn num_params() -> usize;
}
```

This implies that to call this function, you need to be able to know the amount of arguments it expects at COMPILE time. This is not compatibile with dynamic functions, and would require a lot of code generation to make work with BMS.
Languages with no support for dynamic functions are not compatible with BMS.

## Interoperability with Rust

Not all languages can easilly be called from Rust. Lua has a wonderful crate which works out the ffi and safety issues for us. But not all languages have this luxury. If you can't call it from Rust easilly and there is no existing crate that can do it for you, integrating with BMS might not be the best idea.

