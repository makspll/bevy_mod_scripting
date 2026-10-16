---@meta
---@module "Bool"


---@class Bool
--- A boolean value
local Bool = {}



---@class Char
--- An 8-bit character
local Char = {}



---@class DynamicFunction
--- A callable dynamic function
local DynamicFunction = {}



---@class DynamicFunctionMut
--- A stateful and callable dynamic function
local DynamicFunctionMut = {}



---@class F32
--- A 32-bit floating point number
local F32 = {}



---@class F64
--- A 64-bit floating point number
local F64 = {}



---@class FunctionCallContext
--- Function call context, if accepted by a function, means the function can access the world in arbitrary ways.
local FunctionCallContext = {}



---@class I128
--- A signed 128-bit integer
local I128 = {}



---@class I16
--- A signed 16-bit integer
local I16 = {}



---@class I32
--- A signed 32-bit integer
local I32 = {}



---@class I64
--- A signed 64-bit integer
local I64 = {}



---@class I8
--- A signed 8-bit integer
local I8 = {}



---@class Isize
--- A signed pointer-sized integer
local Isize = {}



---@class OsString
--- A heap allocated OS string
local OsString = {}



---@class PathBuf
--- A heap allocated file path
local PathBuf = {}



---@class ReflectReference
--- A reference to a reflectable type
local ReflectReference = {}



---@class ScriptValue
--- A value representing the union of all representable values
local ScriptValue = {}



---@class StaticTypeRef
--- A type itself, used to reach its static members, i.e. `Vec3` in `Vec3.new{}`. Functions on it are shared by all types
local StaticTypeRef = {}



---@class Str
--- A string slice
local Str = {}



---@class String
--- A heap allocated string
local String = {}



---@class U128
--- An unsigned 128-bit integer
local U128 = {}



---@class U16
--- An unsigned 16-bit integer
local U16 = {}



---@class U32
--- An unsigned 32-bit integer
local U32 = {}



---@class U64
--- An unsigned 64-bit integer
local U64 = {}



---@class U8
--- An unsigned 8-bit integer
local U8 = {}



---@class Usize
--- An unsigned pointer-sized integer
local Usize = {}




---@param arg1 integer
---@return integer
function hello_world(arg1) end


---@type any
--- A static class allowing calls through the "." operator only. 
my_static_instance = {}

---@type ladfile_builder.UnitType[]
--- An global instance of this type
my_non_static_instance = {}

---@type table<string, string | string>
--- An global instance of this type
map = {}

