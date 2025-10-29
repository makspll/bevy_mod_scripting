---@meta
---@module "PlainStructType"

---@class PlainStructType
---  I am a simple plain struct type
---@field  int_field ? integer
PlainStructType = {}

---@package
---@param p1 PlainStructType 

---@param p2 integer 

---@return any
function PlainStructType:plain_struct_function(p1,p2) end


---@class EnumType

EnumType = {}


---@class TupleStructType
---  I am a tuple test type
---@field  [1] ? integer
---@field  [2] ? string
TupleStructType = {}


---@class UnitType
---  I am a unit test type
UnitType = {}




---@type GenericStructType
--- A static class allowing calls through the "." operator only. 
my_static_instance = {}

---@type UnitType[]
--- An global instance of this type
my_non_static_instance = {}

---@type table<string, string | string>
--- An global instance of this type
map = {}

