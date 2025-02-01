# ScriptQueryResult

The result of a query, built by the query builder.

## entity

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptQueryResult` | The query result |

Returns:

| Return | Description |
| ---  | --- |
| `Entity` | The entity |

```lua
local entity = result:entity()
```

## components

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptQueryResult` | The query result |

Returns:

| Return | Description |
| ---  | --- |
| `Vec<ReflectReference>` | The components |

```lua
for _, component in pairs(result:components()) do
    print(component)
end
```