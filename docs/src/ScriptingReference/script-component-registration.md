# ScriptComponentRegistration

A reference to a component type's registration, in general think of this as a handle to a type.

## type_name

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptComponentRegistration` | The type registration as returned by `get_type_by_name` |

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
| `s` | `ScriptComponentRegistration` | The type registration as returned by `get_type_by_name` |

Returns:

| Return | Description |
| ---  | --- |
| `String` | The short name |

```lua
local name = MyType:short_name()
```