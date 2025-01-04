# ScriptTypeRegistration

A reference to a type registration, in general think of this as a handle to a type.

## type_name

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` |

Returns:

| Return | Description |
| ---  | --- |
| `String` | The type name |

```lua
local name = MyType:type_name()
```

## short_name

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` |

Returns:

| Return | Description |
| ---  | --- |
| `String` | The short name |

```lua
local name = MyType:short_name()
```

## is_resource

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` |

Returns:

| Return | Description |
| ---  | --- |
| `bool` | `true` if the type is a resource, otherwise `false` |

```lua
if MyType:is_resource() then
    print("MyType is a resource")
end
```

## is_component

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` |

Returns:

| Return | Description |
| ---  | --- |
| `bool` | `true` if the type is a component, otherwise `false` |

```lua
if MyType:is_component() then
    print("MyType is a component")
end
```
