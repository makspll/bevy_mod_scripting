## Sprite spawn

In this page, we will show how to spawn a sprite using lua code by creating new lua bindings.
The following will be showcased:
- Rust plugin that adds a `add_sprite_to_entity`
  - A add_sprite_to_entity that 
    - Queries a resource
    - Creates and inserts a component in an existing entity
- Lua code that uses the add_sprite_to_entity function to show a sprite on screen

### Rust plugin

This should be plugged in your code with `app.add_plugins(MapIconLoader);`

```rust,ignore
use bevy::log::error;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_mod_scripting::core::bindings::{
    FunctionCallContext, GlobalNamespace, NamespaceBuilder, ReflectReference,
    ScriptComponentRegistration, ScriptResourceRegistration, ScriptTypeRegistration, Val,
};

pub struct MapIconLoader;

#[derive(Resource, Default, Reflect, Clone)]
pub struct IconLoaded {
    hash: HashMap<String, Handle<Image>>,
}

impl Plugin for MapIconLoader {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.init_resource::<IconLoaded>();
        app.register_type::<IconLoaded>();
        let world = app.world_mut();
        NamespaceBuilder::<GlobalNamespace>::new_unregistered(world)
            .register("add_sprite_to_entity", add_sprite_to_entity);
    }
}

fn setup(mut commands: Commands, mut icon_loaded: ResMut<IconLoaded>, asset_server: Res<AssetServer>) {
    icon_loaded.hash.insert(
        "corvette".to_string(),
        asset_server.load("map_icons/corvette.png"),
    );
}

fn add_sprite_to_entity(ctx: FunctionCallContext, entity: Val<Entity>, icon: String) {
    let Ok(world) = ctx.world() else {
        error!("Could not access world in add_sprite_to_entity.");
        return;
    };

    let icon_loaded: ScriptTypeRegistration = world.get_type_by_name("IconLoaded").unwrap();
    let icon_loaded: ScriptResourceRegistration =
        world.get_resource_type(icon_loaded).unwrap().unwrap();
    let icon_loaded: ReflectReference = world
        .get_resource(icon_loaded.resource_id())
        .unwrap()
        .unwrap();

    let icon_loaded: IconLoaded = icon_loaded.downcast(ctx.world().unwrap()).unwrap();
    let Some(image) = icon_loaded.hash.get(&icon) else {
        error!("Image {icon} not loaded.");
        return;
    };
    let sprite: Sprite = Sprite::from_image(image.clone());
    let binding = world.allocator();

    let sprite_reference = {
        let mut allocator = (&binding).write();
        ReflectReference::new_allocated(sprite, &mut allocator)
    };

    let sprite_registration: ScriptTypeRegistration = world.get_type_by_name("Sprite").unwrap();
    let sprite_registration: ScriptComponentRegistration = world
        .get_component_type(sprite_registration)
        .unwrap()
        .unwrap();

    let Ok(_) = world.insert_component(*entity, sprite_registration, sprite_reference) else {
        error!("Unable to insert map icon {icon} on entity {}.", *entity);
        return;
    };
}
```

### Lua code

Once the rust plugin is added, the following lua code should instanciate an entity with the png file `assets/map_icons/corvette.png`.

```lua
    local entity = world.spawn()

    local transform = construct(types.Transform, {})
    transform.translation.x = 0;
    transform.translation.y = 0;
    transform.scale.x = 1;
    transform.scale.y = 1;
    transform.scale.z = 1;

    world.insert_component(entity, types.Transform, transform)

    add_sprite_to_entity(entity, "corvette")
```