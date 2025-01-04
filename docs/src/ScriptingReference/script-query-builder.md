# ScriptQueryBuilder

The query builder is used to build queries for entities with specific components. Can be used to interact with arbitrary entities in the world.

## with

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptQueryBuilder` | The query builder |
| `with` | `ScriptTypeRegistration` | The component to include in the query |

Returns:

| Return | Description |
| ---  | --- |
| `ScriptQueryBuilder` | The updated query builder |

```lua
query:with(MyType):with(MyOtherType)
```

## without

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptQueryBuilder` | The query builder |
| `without` | `ScriptTypeRegistration` | The component to exclude from the query |

Returns:

| Return | Description |
| ---  | --- |
| `ScriptQueryBuilder` | The updated query builder |

```lua
query:without(MyType):without(MyOtherType)
```

## build

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptQueryBuilder` | The query builder |

Returns:

| Return | Description |
| ---  | --- |
| `Vec<ScriptQueryResult>` | The query results |

```lua
local results = query.build()
for _, result in pairs(results) do
    print(result)
end
```
