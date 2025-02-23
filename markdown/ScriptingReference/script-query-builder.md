# ScriptQueryBuilder

The query builder is used to build queries for entities with specific components. Can be used to interact with arbitrary entities in the world.

## component

Adds a component to the query, this will be accessible in the query results under the index corresponding to the index of this component in the query.

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptQueryBuilder` | The query builder |
| `component` | `ScriptComponentRegistration` | The component to query for |

Returns:

| Return | Description |
| ---  | --- |
| `ScriptQueryBuilder` | The updated query builder |

```lua
query:component(MyType):component(MyOtherType)
```

## with

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ScriptQueryBuilder` | The query builder |
| `with` | `ScriptComponentRegistration` | The component to include in the query |

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
| `without` | `ScriptComponentRegistration` | The component to exclude from the query |

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
local results = query:build()
for _, result in pairs(results) do
    print(result)
end
```
