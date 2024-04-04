# Architecture

## Reflection 

`bevy_mod_scripting` first and foremost relies on `Reflection`, a feature of Bevy which allows us to interact with type erased data. This is the foundation of the scripting system, as it allows us to interact with the Bevy ECS without knowing the exact types of the components/resources our scripts will be interacting with at compile time.

Normally in Bevy, you would define your components and resources as structs, and then use them in your systems. This is very powerful but also very limiting, as it requires you to know the exact types of the components/resources you will be interacting with at compile time. This is where [`Reflection`](https://docs.rs/bevy_reflect/0.13.1/bevy_reflect/) comes in. 

Bevy provides us with a [`TypeRegistry`](https://docs.rs/bevy_reflect/0.13.1/bevy_reflect/struct.TypeRegistry.html), which is essentially just a map from type ids to [`TypeRegistrations`](https://docs.rs/bevy_reflect/0.13.1/bevy_reflect/struct.TypeRegistration.html). A `TypeRegistration` is a container for all sorts of metadata about the type but most importantly it allows us to query [`TypeData`](https://docs.rs/bevy_reflect/0.13.1/bevy_reflect/trait.TypeData.html) of any type which was previously registered via the `TypeRegistry`.

How is this useful ? Well it allows us to register arbitrary information including function pointers which we can then retrieve given just a `TypeId`. This is exactly what we do with [`ReflectProxyable`](https://docs.rs/bevy_mod_scripting/0.3.0/bevy_mod_scripting/api/lua/struct.ReflectLuaProxyable.html), the interface between Bevy and Lua:

```rust,ignore
pub fn ref_to_lua<'lua>(
    &self,
    ref_: ReflectReference,
    lua: &'lua Lua
) -> Result<Value<'lua>, Error>

pub fn apply_lua<'lua>(
    &self,
    ref_: &mut ReflectReference,
    lua: &'lua Lua,
    new_val: Value<'lua>
) -> Result<(), Error>
```

A `ReflectProxyable` `TypeData` is registered for every type which we want to have custom Lua bindings for. With this we can represent any Reflectable type in any way we want in Lua. For example we can represent a `Vec3` as a table with `x`, `y`, `z` fields, or we can represent it as a userdata with a metatable which has `__index` and `__newindex` metamethods. The best part about this is we do not need to even own the types we are adding this `TypeData` for! This bypasses the pesky orphan rule and allows us to add custom Lua bindings for any type in Bevy. 

Note: for your own types you can do this by deriving `Reflect` and adding a `reflect(LuaProxyable)` attribute like so:

```rust,ignore
#[derive(Reflect)]
#[reflect(LuaProxyable)]
pub struct MyType {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl LuaProxyable for MyType {  
    // ...
}
```

Now when you register your type with the `AppTypeRegistry` it will automatically have a `ReflectLuaProxyable` `TypeData` registered for it! You must not forget to register your type with:

```rust,ignore
app.register_type::<MyType>();
```

## Script References

All accesses to the bevy world are done via `ReflectReference` types which look like this:

```rust,ignore
pub struct ReflectReference {
    /// The reflection path from the root
    pub(crate) path: ReflectionPath,
    pub(crate) world_ptr: WorldPointer,
}
```

I.e. they are essentially just a path to the data in the Bevy world. This allows us to have a reference to a piece of data in the Bevy world which can be passed around and modified in Lua safely.

The `ReflectionPath` itself consists of a "base" reference and a list of path segments. Most interesting of which is the base:

```rust,ignore
pub(crate) enum ReflectBase {
    /// A bevy component reference
    Component {
        comp: ReflectComponent,
        entity: Entity,
    },
    /// A bevy resource reference
    Resource { res: ReflectResource },

    /// A script owned reflect type (for example a vector constructed in lua)
    ScriptOwned { val: Weak<RwLock<dyn Reflect>> },
}
```

Given a valid base and a valid path we should always be able to get a valid reference to a piece of data in the Bevy world. Note we make use of other `TypeData` here, i.e. `ReflectComponent` and `ReflectResource` which store function pointers for the specific types of components/resources we are dealing with that allow us to interact with them. For example `ReflectComponent` let's us call: 

```rust,ignore
pub fn reflect<'a>(
    &self,
    entity: EntityRef<'a>
) -> Option<&'a (dyn Reflect + 'static)>
```

To retrieve a reflect reference to our component on a specific entity!

You might be wondering how exactly we get a `TypeId` from a script in the first place, and the answer is we use a simple String type name! The journey begins in our custom `World` UserData:

```rust,ignore
        methods.add_method("get_type_by_name", |_, world, type_name: String| {
            let w = world.read();

            let registry: &AppTypeRegistry = w.get_resource().unwrap();

            let registry = registry.read();

            Ok(registry
                .get_with_short_type_path(&type_name)
                .or_else(|| registry.get_with_type_path(&type_name))
                .map(|registration| LuaTypeRegistration::new(Arc::new(registration.clone()))))
        });
```

Given a String type name like: `my_crate::MyType` we can then retrieve both `TypeId` and `TypeRegistration` structs, which we can use to retrieve any `TypeData` we need!


## Bevy to Lua Bridge 

Now finally our `ReflectReference` type has a custom `IntoLua` implementation which does the following:
- Check the type has a `ReflectLuaProxyable` `TypeData`
- If it does, call `ref_to_lua` on it and generate the Lua representation of the data
- If it does not, default to a "vanilla" representation of the data i.e. a [`ReflectedValue`](https://docs.rs/bevy_mod_scripting/0.3.0/bevy_mod_scripting/api/struct.ReflectedValue.html) which is a simple wrapper around a `ReflectReference`. It uses pure reflection to provide `__index` and `__newindex` metamethods for the data.

```rust,ignore
    fn into_lua(self, ctx: &'lua Lua) -> mlua::Result<Value<'lua>> {
        let world = self.world_ptr.clone();
        let world = world.read();

        let typedata = &world.resource::<AppTypeRegistry>();
        let g = typedata.read();

        let type_id = self.get(|s| s.type_id())?;
        if let Some(v) = g.get_type_data::<ReflectLuaProxyable>(type_id) {
            v.ref_to_lua(self, ctx)
        } else {
            ReflectedValue { ref_: self }.into_lua(ctx)
        }
    }
```

Note that assigning to bevy via ReflectedValue's will check if the value we're trying to assign has a `ReflectLuaProxyable` type data, and if it does it uses it's `apply_lua` method to apply the new value to the `ReflectReference`, if it does not it expects it to be another `ReflectedValue` and will clone then apply it to itself using pure reflection.

All primitive data types will have a `ReflectLuaProxyable` type data registered for them via their `FromLua` and `Clone` implementations.


## Proxy macros
We provide a set of macros to make it easier to define custom Lua bindings for your types. For example:

```rust,ignore
#[derive(LuaProxy, Reflect, Resource, Default, Debug, Clone)]
#[reflect(Resource, LuaProxyable)]
#[proxy(
    derive(clone),
    functions[
        r#"
        #[lua(kind="MutatingMethod")]
        fn set_my_string(&mut self, another_string: Option<String>);
        "#,
        r#"
        #[lua(kind="MutatingMethod")]
        fn set_with_another(&mut self, #[proxy] another: Self);
        "#,
        r#"
        #[lua(kind="Method")]
        fn get_my_string(&self) -> String;
        "#,
        r#"
        #[lua(kind="Method",raw)]
        fn raw_method(&self, ctx : &Lua) -> Result<String, _> {
            let a = ctx.globals().get::<_,String>("world").unwrap();
            let a = self.inner()?;
            Ok("".to_owned())
        }
        "#,
        r#"
        #[lua(kind="MetaMethod", metamethod="ToString")]
        fn to_string(&self) -> String {
            format!("{:#?}", _self)
        }
        "#
    ])
    ]
pub struct MyProxiedStruct {
    my_string: String,
}
```

will generate a `LuaMyProxiedStruct` which will act as the Lua representation of `MyProxiedStruct`. It will have the following methods: 
- `set_my_string` which will set the `my_string` field of the struct
- `set_with_another` which will set the struct to be equal to another struct
- `get_my_string` which will return the `my_string` field of the struct
- `ToString` metamethod which will return a string representation of the struct

It will also implement `UserData` for the proxy, meaning it can be passed around in Lua as a first class citizen. And it will implement `LuaProxyable` for `MyProxiedStruct`, meaning you can register your type and have it work in Lua with no extra work!

## Bevy API Generation

A good scripting system should be able to interact with the Bevy API as well as the user's own types. We provide a way to generate Lua bindings for the Bevy API using a rustc plugin. We scrape the Bevy codebase and generate proxy macro invocations like the one above for every appropriate `Reflect` implementing type, and package them in `APIProvider` structs which you can use to provide the Bevy API to your Lua scripts.

This generator is a work in progress but it is designed with the possibility of generating bindings for ANY crate in mind. It is not limited to Bevy, and can be used to generate bindings for any crate which uses `Reflect` types. In theory you should be able to use the CLI to generate your own bindings without writing macros yourself! See the [`bevy_api_gen`](crates/bevy_api_gen/readme.md) crate for more information.


