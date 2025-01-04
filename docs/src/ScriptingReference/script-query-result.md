# ScriptQueryResult

The result of a query, built by the query builder.

## entity

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptQueryResult` | The query result |


| Return | Description |
| ---  | --- |
| `Entity` | The entity |

```lua
local entity = result:entity()
```

## components

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