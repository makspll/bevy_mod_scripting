# Writing Bindings

The bevy bindings shipped with BMS are hand written, using the `#[script_bindings]` macro (see [Controlling Script Bindings](../Summary/controlling-script-bindings.md)).
They live in `crates/bindings/bevy_mod_scripting_bevy_<category>_bindings`, one crate per area of functionality (i.e. `math`, `ecs`, `ui`) rather than one per upstream bevy crate.

The aim is a small, well documented scripting surface, which reads naturally from a script and compiles quickly.

## What to bind

Every reflected type is already usable from scripts without any bindings:
- fields are read and written through references, i.e. `transform.translation.x = 2`
- every type can be constructed with `Type.new({...})`, see [Constructing Arbitrary Types](../ScriptingReference/constructing-arbitrary-types.md)
- enum variants are available as `Type.Variant`
- containers support `len`, `iter`, `push` etc., and every value can be printed

So only bind what scripts can't already do, or can't do comfortably:
- methods with behaviour, i.e. `Vec3:normalize()` or `Transform:look_at(...)`
- constructors for types which are built often (vectors, colors), or whose fields are private (i.e. `Name`), or whose meaning isn't obvious from their fields (i.e. `Quat.from_rotation_y`)
- operators, by registering functions named `add`, `sub`, `mul`, `div`, `rem`, `neg`, `pow`, `eq` or `lt`

Do not write getters or setters for public fields.

## Performance

- Take non primitive arguments as `R<T>` or `M<T>`. These are a downcast of the reference the script holds, while `V<T>` rebuilds the value through reflection. Clone natively if you need ownership.
- Return a reference with an extended path when returning a part of the receiver; it costs nothing, and writes to it reach the original value. Returning `V<T>` allocates a new value.
- Register one function per name. Overloads are tried one by one, so for mixed operands accept a `Union`, i.e. `rhs: Union<R<Vec3>, f32>`.
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
local v = Vec3.new(1, 2, 3)
assert(v.x == 1)
```

```multi-lang-example,rhai
let v = Vec3.new_.call(1.0, 2.0, 3.0);
assert(v.x == 1.0);
```
````

which renders as a single block with a tab per language:

```multi-lang-example,lua
local v = Vec3.new(1, 2, 3)
assert(v.x == 1)
```

```multi-lang-example,rhai
let v = Vec3.new_.call(1.0, 2.0, 3.0);
assert(v.x == 1.0);
```

Examples should run as written and check their own results with `assert`, so that they can be run as tests in the future.
They run in the same app as the [script tests](#tests) (`assets/tests/scenario.bmsscenario`).
If an example needs a different setup, i.e. a window, name a scenario with the `scenario` attribute, i.e. `multi-lang-example,lua,scenario=window`, which refers to `assets/doc_examples/window.bmsscenario`.

## Tests

Each bindings crate has its script tests under `assets/tests/bindings/<category>/`, with a Lua and a Rhai script per type or topic, i.e. `assets/tests/bindings/math/vec3.lua` and `assets/tests/bindings/math/vec3.rhai`.
Run them with:

```bash
cargo test --test script_tests --features lua54,rhai -- bindings/math
```
