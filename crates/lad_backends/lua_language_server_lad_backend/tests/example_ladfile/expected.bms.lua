---@meta
---@module "bms.List"


---@class bms.List<T> : ReflectReference
--- A list field of a value, i.e. a `Vec`: a reference to the list, indexed from 1.
---@field  [integer] T
local List = {}

--- Appends a value to the list.
---@generic T
---@param list bms.List<T>
---@param value T
---@return nil
function List.push(list,value) end

--- Removes the last value of the list and returns it.
---@generic T
---@param list bms.List<T>
---@return T
function List.pop(list) end

--- Inserts a value at the index, shifting the values after it.
---@generic T
---@param list bms.List<T>
---@param index integer
---@param value T
---@return nil
function List.insert(list,index,value) end

--- Removes the value at the index and returns it.
---@generic T
---@param list bms.List<T>
---@param index integer
---@return T
function List.remove(list,index) end

--- The number of values in the list.
---@generic T
---@param list bms.List<T>
---@return integer
function List.len(list) end

--- Removes every value of the list.
---@generic T
---@param list bms.List<T>
---@return nil
function List.clear(list) end


