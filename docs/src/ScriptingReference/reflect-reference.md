# ReflectReference

ReflectReferences are simply references to date living either:
- In a component
- In a resource
- In the allocator

Reflect references contain a standard interface which operates over the reflection layer exposed by `Bevy` and also provides a way to call various dynamic functions registered on the underlying pointed to data.

## display_ref

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ReflectReference` | The reference to display |

Returns:

| Return | Description |
| ---  | --- |
| `String` | The reference in string format |

```lua
print(ref:display_ref())
print(ref)
```

## display_value

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ReflectReference` | The reference to display |

Returns:

| Return | Description |
| ---  | --- |
| `String` | The value in string format |

```lua
print(ref:display_value())
```

## get
The index function, allows you to index into the reflect reference.

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `key` | `ScriptValue` | The key to get the value for |

Returns:

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

## set

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `key` | `ScriptValue` | The key to set the value for |
| `value` | `ScriptValue` | The value to set |

Returns:

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

## push
Generic push method, if the underlying type supports it, will push the value into the end of the reference.

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `value` | `ScriptValue` | The value to push |

```lua
ref:push(value)
```

## pop
Generic pop method, if the underlying type supports it, will pop the value from the end of the reference.

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ReflectReference` | The reference to pop from |

Returns:

| Return | Description |
| ---  | --- |
| `ScriptValue` | The popped value |

```lua
local value = ref:pop()
```

## insert
Generic insert method, if the underlying type supports it, will insert the value at the key.

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `key` | `ScriptValue` | The key to insert the value for |
| `value` | `ScriptValue` | The value to insert |

```lua
ref:insert(key, value)
```

## clear
Generic clear method, if the underlying type supports it, will clear the referenced container type.

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ReflectReference` | The reference to clear |


```lua
ref:clear()
```

## len
Generic length method, if the underlying type supports it, will return the length of the referenced container or length relevant to the type itself (number of fields etc.).

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ReflectReference` | The reference to get the length of |

Returns:

| Return | Description |
| ---  | --- |
| `usize` | The length |

```lua
length = ref:len()
```

## remove
Generic remove method, if the underlying type supports it, will remove the value at the key.

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `key` | `ScriptValue` | The key to remove the value for |

Returns:

| Return | Description |
| ---  | --- |
| `ScriptValue` | The removed value |

```lua
local value = ref:remove(key)
```

## iter
The iterator function, returns a function which can be called to iterate over the reference.

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `s` | `ReflectReference` | The reference to iterate over |

Returns:

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
