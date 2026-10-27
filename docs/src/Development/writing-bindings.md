# Writing Bindings

The bevy bindings shipped with BMS are hand written, using the `#[script_bindings]` macro (see [Controlling Script Bindings](../Summary/controlling-script-bindings.md)).
They live in `crates/bindings/bevy_mod_scripting_bevy_<category>_bindings`, one crate per area of functionality (i.e. `math`, `ecs`, `ui`) rather than one per upstream bevy crate.

The aim is a small, well documented scripting surface, which reads naturally from a script and compiles quickly.

## Namespaces

Scripts reach types through the namespace of their category, i.e. `import_namespace("math").Vec3` (see [Namespaces](../ScriptingReference/namespaces.md)). So each bindings crate:
- passes `namespace = "<category>"` to every `#[script_bindings]` attribute, except on `impl World` blocks and on functions shared by all types (`impl StaticTypeRef`), which would otherwise move those BMS types into the category
- calls `set_crate_script_namespaces` from its plugin's `finish`, with the bevy crates it covers, so their types without bindings land in the same namespace

## Generic types

Reflection only sees instantiations of generic types (`Time<Virtual>`, `Time<Fixed>`), which would be documented as unrelated types. Declare the generic types of the crate from its plugin, instantiated with the marker `A` in place of their parameter, in the namespace of the category:

```rust,ignore
use bevy_mod_scripting_bindings::function::generics::{A, GenericTypesAppExt};

app.register_generic_type::<Time<A>>("time", &["T"], "A clock, i.e. `Time<Virtual>`.");
```

The fields of type `A` are then documented as being of the parameter `T`, and the instantiations as `Time<T>` applied to their arguments. Only declare types whose parameter is part of what they are (`Handle`, `Time`, `ButtonInput`), not containers like `Arc<T>`.

Functions which return values of the type they are given are declared generic in their signature, with the same marker:

```rust,ignore
fn get_component(
    ctxt: FunctionCallContext,
    entity: V<Entity>,
    registration: TypeOf<V<ScriptComponentRegistration>, A>,
) -> Result<Option<ValueOf<A>>, InteropError>
```

`TypeOf<R, A>` is a type passed as `R`, `ValueOf<A>` a reference to a value of it. They only change the documentation, i.e. `world.get_component(entity, Transform)` is a `Transform` in Lua.

## What to bind

Every reflected type is already usable from scripts without any bindings:
- fields are read and written through references, i.e. `transform.translation.x = 2`
- every type can be constructed with `Type.new({...})`, see [Constructing Arbitrary Types](../ScriptingReference/constructing-arbitrary-types.md)
- enum variants are available as `Type.Variant`
- containers support `len`, `iter`, `push` etc., and every value can be printed

So only bind what scripts can't already do, or can't do comfortably:
- methods with behaviour, i.e. `Vec3:normalize()` or `Transform:look_at(...)`
- constructors for types which are built often (vectors, colors), or whose fields are private (i.e. `Name`), or whose meaning isn't obvious from their fields (i.e. `Quat.from_rotation_y`)
- operators, by registering functions named `add`, `sub`, `mul`, `div`, `rem`, `neg`, `pow`, `eq` or `lt`. With a number on the left, i.e. `2 * v`, `add` and `mul` are called with the operands swapped, the others call `rsub`, `rdiv`, `rrem` or `rpow` on the right operand, which you register where they make sense

Do not write getters or setters for public fields.

## Performance

- Take non primitive arguments as `R<T>` or `M<T>`. These are a downcast of the reference the script holds, while `V<T>` rebuilds the value through reflection. Clone natively if you need ownership.
- Return a reference with an extended path when returning a part of the receiver; it costs nothing, and writes to it reach the original value. Returning `V<T>` allocates a new value.
- There is one function per name: registering a name again replaces the function. For mixed operands accept a `Union`, i.e. `rhs: Union<R<Vec3>, f32>`.
- Mutate in place by taking `M<Self>`; pure functions return new values.

## Documentation

Doc comments end up in the generated docs and in editor declaration files, and are read by script writers rather than Rust developers:
- start with one short sentence, and only describe the arguments when they are not obvious
- avoid Rust specific terms, i.e. say "or nil" rather than `Option`
- link bevy types using their full Rust path, i.e. ``[`Vec3`](glam::Vec3)``, these will be resolved to the bevy docs

### Examples

Keep examples short and only add them where they help. Write one block per language, one after the other, using the `multi-lang-example` code block attribute followed by the language:

````markdown
```multi-lang-example,lua
local Vec3 = import_namespace("math").Vec3
local v = Vec3.new(1, 2, 3)
assert(v.x == 1)
```

```multi-lang-example,rune
let Vec3 = import_namespace("math").Vec3;
let v = Vec3.new(1, 2, 3);
assert!(v.x == 1.0);
```
````

which renders as a single block with a tab per language:

```multi-lang-example,lua
local Vec3 = import_namespace("math").Vec3
local v = Vec3.new(1, 2, 3)
assert(v.x == 1)
```

```multi-lang-example,rune
let Vec3 = import_namespace("math").Vec3;
let v = Vec3.new(1, 2, 3);
assert!(v.x == 1.0);
```

Examples are tests: they must run as written and check their own results with `assert`.
Every `multi-lang-example` block in the doc comments under `crates/` and in the book is run by the `doc_example_tests` test, with the scenario in `assets/doc_examples/default.bmsscenario` (the same app as the [script tests](#tests)):

```bash
cargo test --test doc_example_tests --features lua54,rune,bevy_math_bindings -- math
```

Examples of a bindings crate whose feature is disabled are ignored.
If an example needs a different setup, i.e. a window, name a scenario with the `scenario` attribute, i.e. `multi-lang-example,lua,scenario=window`, which refers to `assets/doc_examples/window.bmsscenario`.
Mark examples which can't run in a test app with `ignore`, i.e. `multi-lang-example,lua,ignore`, and say what they need in the surrounding docs.

## Tests

Each bindings crate has its script tests under `assets/tests/bindings/<category>/`, with a script per language per type or topic, i.e. `assets/tests/bindings/math/vec.lua` and `assets/tests/bindings/math/vec.rn`.
Run them with:

```bash
cargo test --test script_tests --features lua54,rune -- bindings/math
```
