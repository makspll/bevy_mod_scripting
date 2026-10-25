---@meta
---@module "HoldsOpaqueType"


---@class ladfile_builder.HoldsOpaqueType : ReflectReference
---  I hold an instantiation of a generic type
---@field  generic? ladfile_builder.OpaqueGenericType<integer>
local HoldsOpaqueType = {}

---@param arg1 ladfile_builder.HoldsOpaqueType
---@param arg2 ladfile_builder.OpaqueGenericType<integer>
---@return ladfile_builder.OpaqueGenericType<integer>
function HoldsOpaqueType.swap_generic(arg1,arg2) end



---@class ladfile_builder.PlainStructType : ReflectReference
---  I am a simple plain struct type
---@field  int_field? integer
local PlainStructType = {}

---@param arg1 ladfile_builder.PlainStructType
---@param arg2 integer
---@return any
function PlainStructType.plain_struct_function(arg1,arg2) end



---@class ladfile_builder.GenericStructType : ReflectReference
---  I am a struct
---@field  field? integer
---@field  field2? integer
local GenericStructType = {}



---@class ladfile_builder.GenericStructType.Statics
--- The static functions and variants of `ladfile_builder.GenericStructType`, i.e. `GenericStructType.function()` in scripts.
local GenericStructType_statics = {}

---@param ref_ ReflectReference
--- I am some docs for argument 1
---@param tuple [integer, string]
--- I am some docs for argument 2
---@param option_vec_ref_wrapper? ladfile_builder.EnumType[] | nil
--- I am some docs for argument 3
---@return integer
function GenericStructType_statics.hello_world(ref_,tuple,option_vec_ref_wrapper) end



---@class ladfile_builder.EnumType : ReflectReference
local EnumType = {}



---@class ladfile_builder.EnumType.Statics
--- The static functions and variants of `ladfile_builder.EnumType`, i.e. `EnumType.function()` in scripts.
---@field  Unit ladfile_builder.EnumType.Unit
---@field  Struct ladfile_builder.EnumType.Struct
---@field  TupleStruct ladfile_builder.EnumType.TupleStruct
local EnumType_statics = {}



---@class ladfile_builder.EnumType.Unit : EnumVariantRef
--- The `Unit` variant of `ladfile_builder.EnumType`.
EnumType_statics.Unit = {}



---@class ladfile_builder.EnumType.Struct : EnumVariantRef
--- The `Struct` variant of `ladfile_builder.EnumType`.
EnumType_statics.Struct = {}



---@class ladfile_builder.EnumType.TupleStruct : EnumVariantRef
--- The `TupleStruct` variant of `ladfile_builder.EnumType`.
EnumType_statics.TupleStruct = {}



---@class ladfile_builder.TupleStructType : ReflectReference
---  I am a tuple test type
---@field  [1] integer
---@field  [2] string
local TupleStructType = {}



---@class ladfile_builder.UnitType : ReflectReference
---  I am a unit test type
local UnitType = {}



---@class ladfile_builder.OpaqueGenericType<T> : ReflectReference
--- I am the documentation of a generic type.




---@class namespace.ladfile_builder
--- The `ladfile_builder` namespace, as returned by `import_namespace("ladfile_builder")`.
---@field  EnumType ladfile_builder.EnumType.Statics | StaticTypeRef<ladfile_builder.EnumType>
---@field  GenericStructType ladfile_builder.GenericStructType.Statics | StaticTypeRef<ladfile_builder.GenericStructType>
---@field  HoldsOpaqueType StaticTypeRef<ladfile_builder.HoldsOpaqueType>
---@field  PlainStructType StaticTypeRef<ladfile_builder.PlainStructType>
---@field  TupleStructType StaticTypeRef<ladfile_builder.TupleStructType>
---@field  UnitType StaticTypeRef<ladfile_builder.UnitType>
---@field  map table<string, string | string>
---@field  my_non_static_instance ladfile_builder.UnitType[]
---@field  my_static_instance any
local namespace_ladfile_builder = {}


