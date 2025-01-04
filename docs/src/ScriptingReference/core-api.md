# Core Bindings

The core bindings are manually written utilities for interacting with the `Bevy` world and everything contained within it. These bindings are used to create and manipulate entities, components, resources, and systems.

Every language BMS supports will support these.

## World

The `World` is the entry point for interacting with `Bevy`. It is provided to scripts under either the `world` or `World` static variable. 

### get_type_by_name

| Argument | Type | Description |
| --- | --- | --- |
| `type_name` | `String` | The name of the type to get, this can be either the short type name, i.e. `my_type` or the long name i.e. `my_crate::my_module::my_type` |

| Return | Description |
| ---  | --- |
| `Option<ScriptTypeRegistration>` | The type if it exists, otherwise `None` |

```lua
MyType = world.get_type_by_name("MyType")
if MyType == nil then
    print("MyType not found")
end
```

### get_component

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The entity to get the component from |
| `registration` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` of the component |

| Return | Description |
| ---  | --- |
| `Option<ReflectReference>` | The reference to the component if it exists, otherwise `None` |

```lua
local component = world.get_component(entity, MyType)
if component ~= nil then
    print("found component:" .. component)
end
```

### has_component

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The entity to check the component for |
| `registration` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` of the component |

| Return | Description |
| ---  | --- |
| `bool` | `true` if the entity has the component, otherwise `false` |

```lua
if world.has_component(entity, MyType) then
    print("Entity has MyType")
end
```

### remove_component

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The entity to remove the component from |
| `registration` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` of the component |

```lua
world.remove_component(entity, MyType)
```

### get_resource

| Argument | Type | Description |
| --- | --- | --- |
| `registration` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` of the resource |

| Return | Description |
| ---  | --- |
| `Option<ReflectReference>` | The resource if it exists, otherwise `None` |

```lua
local resource = world.get_resource(MyType)
if resource ~= nil then
    print("found resource:" .. resource)
end
```

### has_resource

| Argument | Type | Description |
| --- | --- | --- |
| `registration` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` of the resource |

| Return | Description |
| ---  | --- |
| `bool` | `true` if the resource exists, otherwise `false` |

```lua
local hasResource = world.has_resource(MyType)
```

### remove_resource

| Argument | Type | Description |
| --- | --- | --- |
| `registration` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` of the resource |

```lua
world.remove_resource(MyType)
```

### add_default_component

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The entity to add the component to |
| `registration` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` of the component |

```lua
world.add_default_component(entity, MyType)
```

### spawn

| Return | Description |
| ---  | --- |
| `Entity` | The spawned entity |

```lua
local entity = world.spawn()
```

### insert_children

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The parent entity |
| `index` | `usize` | The index to insert the children at |
| `children` | `Vec<Entity>` | The children entities to insert |

```lua
world.insert_children(parent, 1, {child1, child2})
```

### push_children

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The parent entity |
| `children` | `Vec<Entity>` | The children entities to push |


```lua
world.push_children(parent, {child1, child2})
```

### get_children

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The parent entity |

| Return | Description |
| ---  | --- |
| `Vec<Entity>` | The children entities |

```lua
local children = world.get_children(parent)
for _, child in pairs(children) do
    print("child: " .. child)
end
```

### get_parent

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The child entity |

| Return | Description |
| ---  | --- |
| `Option<Entity>` | The parent entity if it exists, otherwise `None` |

```lua
local parent = world.get_parent(child)
if parent ~= nil then
    print("parent: " .. parent)
end
```

### despawn

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The entity to despawn |

```lua
world.despawn(entity)
```

### despawn_descendants

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The entity to despawn descendants of |

```lua
world.despawn_descendants(entity)
```

### despawn_recursive

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The entity to despawn recursively |

```lua
world.despawn_recursive(entity)
```

### has_entity

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The entity to check |

| Return | Description |
| ---  | --- |
| `bool` | `true` if the entity exists, otherwise `false` |

```lua
local exists = world.has_entity(entity)
if exists then
    print("entity exists")
end
```

### query

| Argument | Type | Description |
| --- | --- | --- |
| `components` | `Vec<ScriptTypeRegistration>` | The components to query for |

| Return | Description |
| ---  | --- |
| `ScriptQueryBuilder` | The query builder |

```lua
query = world.query({MyType})
```

### exit
Send the exit signal to the application, will gracefully shutdown the application.

```lua
world.exit()
```

## ReflectReference

ReflectReferences are simply references to date living either:
- In a component
- In a resource
- In the allocator

Reflect references contain a standard interface which operates over the reflection layer exposed by `Bevy` and also provides a way to call various dynamic functions registered on the underlying pointed to data.

### display_ref

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ReflectReference` | The reference to display |

| Return | Description |
| ---  | --- |
| `String` | The reference in string format |

```lua
print(ref:display_ref())
print(ref)
```

### display_value

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ReflectReference` | The reference to display |

| Return | Description |
| ---  | --- |
| `String` | The value in string format |

```lua
print(ref:display_value())
```

### get
The index function, allows you to index into the reflect reference.

| Argument | Type | Description |
| --- | --- | --- |
| `key` | `ScriptValue` | The key to get the value for |

| Return | Description |
| ---  | --- |
| `ScriptValue` | The value |

```lua
local value = ref:get(key)
-- same as
local value = ref.key
local value = ref[key]
local value = ref["key"]
-- for tuple structs
local valye = ref._1
```

### set

| Argument | Type | Description |
| --- | --- | --- |
| `key` | `ScriptValue` | The key to set the value for |
| `value` | `ScriptValue` | The value to set |

| Return | Description |
| ---  | --- |
| `ScriptValue` | The result |

```lua
ref:set(key, value)
-- same as
ref.key = value
ref[key] = value
ref["key"] = value
-- for tuple structs
ref._1 = value
```

### push
Generic push method, if the underlying type supports it, will push the value into the end of the reference.

| Argument | Type | Description |
| --- | --- | --- |
| `value` | `ScriptValue` | The value to push |

```lua
ref:push(value)
```

### pop
Generic pop method, if the underlying type supports it, will pop the value from the end of the reference.

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ReflectReference` | The reference to pop from |

| Return | Description |
| ---  | --- |
| `ScriptValue` | The popped value |

```lua
local value = ref:pop()
```

### insert
Generic insert method, if the underlying type supports it, will insert the value at the key.

| Argument | Type | Description |
| --- | --- | --- |
| `key` | `ScriptValue` | The key to insert the value for |
| `value` | `ScriptValue` | The value to insert |

```lua
ref:insert(key, value)
```

### clear
Generic clear method, if the underlying type supports it, will clear the referenced container type.

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ReflectReference` | The reference to clear |


```lua
ref:clear()
```

### len
Generic length method, if the underlying type supports it, will return the length of the referenced container or length relevant to the type itself (number of fields etc.).

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ReflectReference` | The reference to get the length of |

| Return | Description |
| ---  | --- |
| `usize` | The length |

```lua
length = ref:len()
```

### remove
Generic remove method, if the underlying type supports it, will remove the value at the key.

| Argument | Type | Description |
| --- | --- | --- |
| `key` | `ScriptValue` | The key to remove the value for |

| Return | Description |
| ---  | --- |
| `ScriptValue` | The removed value |

```lua
local value = ref:remove(key)
```

### iter
The iterator function, returns a function which can be called to iterate over the reference.

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ReflectReference` | The reference to iterate over |

| Return | Description |
| ---  | --- |
| `ScriptFunctionMut` | The iterator function |

```lua
local iter = ref:iter()
local val = iter()
while val do
    print(val)
    next = iter()
end

-- same as 
for val in pairs(ref) do
    print(val)
end
```

## ScriptTypeRegistration

A reference to a type registration, in general think of this as a handle to a type.

### type_name

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` |

| Return | Description |
| ---  | --- |
| `String` | The type name |

```lua
local name = MyType:type_name()
```

### short_name

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` |

| Return | Description |
| ---  | --- |
| `String` | The short name |

```lua
local name = MyType:short_name()
```

### is_resource

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` |

| Return | Description |
| ---  | --- |
| `bool` | `true` if the type is a resource, otherwise `false` |

```lua
if MyType:is_resource() then
    print("MyType is a resource")
end
```

### is_component

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` |

| Return | Description |
| ---  | --- |
| `bool` | `true` if the type is a component, otherwise `false` |

```lua
if MyType:is_component() then
    print("MyType is a component")
end
```

## ScriptQueryBuilder

The query builder is used to build queries for entities with specific components. Can be used to interact with arbitrary entities in the world.

### with

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptQueryBuilder` | The query builder |
| `with` | `ScriptTypeRegistration` | The component to include in the query |

| Return | Description |
| ---  | --- |
| `ScriptQueryBuilder` | The updated query builder |

```lua
query:with(MyType):with(MyOtherType)
```

### without

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptQueryBuilder` | The query builder |
| `without` | `ScriptTypeRegistration` | The component to exclude from the query |

| Return | Description |
| ---  | --- |
| `ScriptQueryBuilder` | The updated query builder |

```lua
query:without(MyType):without(MyOtherType)
```

### build

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptQueryBuilder` | The query builder |

| Return | Description |
| ---  | --- |
| `Vec<ScriptQueryResult>` | The query results |

```lua
local results = query.build()
for _, result in pairs(results) do
    print(result)
end
```

## ScriptQueryResult

The result of a query, built by the query builder.

### entity

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptQueryResult` | The query result |

| Return | Description |
| ---  | --- |
| `Entity` | The entity |

```lua
local entity = result:entity()
```

### components

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptQueryResult` | The query result |

| Return | Description |
| ---  | --- |
| `Vec<ReflectReference>` | The components |

```lua
for _, component in pairs(result:components()) do
    print(component)
end
```