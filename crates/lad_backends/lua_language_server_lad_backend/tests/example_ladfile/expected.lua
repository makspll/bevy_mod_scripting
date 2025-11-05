---@meta
---@module "PlainStructType"

---@class PlainStructType : ReflectReference
---  I am a simple plain struct type
---@field  int_field ? integer
PlainStructType = {}

---@param p1 PlainStructType 
---@param p2 integer 
---@return any
function PlainStructType:plain_struct_function(p1,p2) end


---@class Bool
--- A boolean value
Bool = {}


---@class Char
--- An 8-bit character
Char = {}


---@class DynamicFunction
--- A callable dynamic function
DynamicFunction = {}


---@class DynamicFunctionMut
--- A stateful and callable dynamic function
DynamicFunctionMut = {}


---@class F32
--- A 32-bit floating point number
F32 = {}


---@class F64
--- A 64-bit floating point number
F64 = {}


---@class FunctionCallContext
--- Function call context, if accepted by a function, means the function can access the world in arbitrary ways.
FunctionCallContext = {}


---@class I128
--- A signed 128-bit integer
I128 = {}


---@class I16
--- A signed 16-bit integer
I16 = {}


---@class I32
--- A signed 32-bit integer
I32 = {}


---@class I64
--- A signed 64-bit integer
I64 = {}


---@class I8
--- A signed 8-bit integer
I8 = {}


---@class Isize
--- A signed pointer-sized integer
Isize = {}


---@class OsString
--- A heap allocated OS string
OsString = {}


---@class PathBuf
--- A heap allocated file path
PathBuf = {}


---@class ReflectReference
--- A reference to a reflectable type
ReflectReference = {}


---@class ScriptValue
--- A value representing the union of all representable values
ScriptValue = {}


---@class Str
--- A string slice
Str = {}


---@class String
--- A heap allocated string
String = {}


---@class U128
--- An unsigned 128-bit integer
U128 = {}


---@class U16
--- An unsigned 16-bit integer
U16 = {}


---@class U32
--- An unsigned 32-bit integer
U32 = {}


---@class U64
--- An unsigned 64-bit integer
U64 = {}


---@class U8
--- An unsigned 8-bit integer
U8 = {}


---@class Usize
--- An unsigned pointer-sized integer
Usize = {}


---@class EnumType : ReflectReference
EnumType = {}


---@class TupleStructType : ReflectReference
---  I am a tuple test type
---@field  [1]  integer
---@field  [2]  string
TupleStructType = {}


---@class UnitType : ReflectReference
---  I am a unit test type
UnitType = {}




---@type any
--- A static class allowing calls through the "." operator only. 
my_static_instance = {}

---@type UnitType[]
--- An global instance of this type
my_non_static_instance = {}

---@type table<string, string | string>
--- An global instance of this type
map = {}

