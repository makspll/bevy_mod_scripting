# Bevy Script API

This is document describes all of the methods and types available to scripts by default.

Every script host must provide this functionality at minimum.

The types presented are rust types, but any type semantically equivalent to the given rust type may be provided by the script.

|Type |  Method Name   |  Arguments   | Output | Description | 
|---| --- | --- | --- | --- |
| Entity | id |  -   |  `u32`   |  id component of entity id   |
| Entity | gen |  -   |  `u32`   |  generation component of entity id   |
| Entity | bits |  -   |  `u64`   |  full id of the entity   |
|---| --- | --- | --- | --- |
| World | new | type name: `String` |  `Box<dyn Reflect>`  | Creates new heap alloacted dynamic struct which can be later populated with fields 
| World | new |   |    |     |
|  |  |   |    |     |
|  |  |   |    |     |
|---| --- | --- | --- | --- |
| Component |  |   |    |     |
|  |  |   |    |     |

