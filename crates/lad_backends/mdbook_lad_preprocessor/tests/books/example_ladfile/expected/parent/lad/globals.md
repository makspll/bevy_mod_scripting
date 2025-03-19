# Globals

## Global Values

Global values that are accessible anywhere inside scripts\. You should avoid naming conflicts with these and trying to overwrite or edit them\.

### Instances

Instances containing actual accessible values\.

| Instance | Type |
| --- | --- |
| `my_non_static_instance` | Vec\<[UnitType](/parent/lad/types/unittype.md)\> |
| `map` | HashMap\<[String](/parent/lad/types/string.md), [String](/parent/lad/types/string.md) \| [String](/parent/lad/types/string.md)\> |

### Static Instances

Static type references, existing for the purpose of typed static function calls\.

| Instance | Type |
| --- | --- |
| `my_static_instance` | StructType\<[usize](/parent/lad/types/usize.md)\> |

