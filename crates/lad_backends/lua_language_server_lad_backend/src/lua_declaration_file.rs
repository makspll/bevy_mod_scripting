use serde::Serialize;
use std::collections::HashMap;

/// Basic primitive types supported by Lua Language Server annotations.
///
/// These correspond to the fundamental Lua types that can be used in type annotations.
///
/// # Example
/// ```lua
/// ---@type nil
/// local my_nil = nil
///
/// ---@type boolean
/// local my_bool = true
///
/// ---@type string
/// local my_string = "hello"
///
/// ---@type number
/// local my_number = 42.5
///
/// ---@type integer
/// local my_int = 42
///
/// ---@type function
/// local my_func = function() end
///
/// ---@type table
/// local my_table = {}
///
/// ---@type thread
/// local my_thread = coroutine.create(function() end)
///
/// ---@type userdata
/// local my_userdata = io.stdout
///
/// ---@type lightuserdata
/// local my_lightuserdata = some_c_pointer
/// ```
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LuaPrimitiveType {
    Nil,
    Boolean,
    String,
    Number,
    Integer,
    Function,
    Table,
    Thread,
    Userdata,
    #[serde(rename = "lightuserdata")]
    LightUserdata,
}

/// Represents a Lua type (can be primitive, alias, union, array, function, etc.)
///
/// Supports all Lua Language Server type annotations including complex types
/// like unions, arrays, generics, and table literals.
///
/// # Examples
/// ```lua
/// ---@type string                    -- Primitive type
/// ---@type MyClass                   -- Alias type
/// ---@type string | number           -- Union type
/// ---@type string[]                  -- Array type
/// ---@type [string, number]          -- Tuple type
/// ---@type table<string, number>     -- Dictionary type
/// ---@type { name: string, age: number } -- Table literal
/// ---@type fun(x: number): string    -- Function type
/// ---@type MyClass<T>                -- Generic type
/// ---@type "left" | "right"          -- Literal types
/// ```
#[derive(Debug, Clone, Serialize)]
pub enum LuaType {
    Primitive(LuaPrimitiveType), // "number", "string", "boolean", etc.
    Alias(String),
    Union(Vec<LuaType>),
    Array(Box<LuaType>),
    Tuple(Vec<LuaType>),
    Dictionary {
        key: Box<LuaType>,
        value: Box<LuaType>,
    },
    TableLiteral(HashMap<String, LuaType>),
    Function(FunctionSignature),
    Generic {
        name: String,
        parent: Option<Box<LuaType>>,
    },
    Literal(String), // for literal types, e.g., '"left"'
    Any,
}

// Function-related definitions
/// Represents a function parameter in Lua Language Server annotations.
///
/// # Examples
/// ```lua
/// ---@param name string            -- Required parameter
/// ---@param age? number             -- Optional parameter
/// ---@param callback fun(): nil    -- Function parameter
/// ---@param ... string             -- Variadic parameter
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct FunctionParam {
    pub name: String,
    pub ty: LuaType,
    pub optional: bool,
    pub description: Option<String>,
}

/// Represents a function signature with comprehensive annotation support.
///
/// # Examples
/// ```lua
/// ---@async                        -- Async function
/// ---@deprecated                   -- Deprecated function
/// ---@nodiscard                    -- Return value should not be ignored
/// ---@package                      -- Package-private function
/// ---@generic T                    -- Generic function
/// ---@param name string            -- Parameters
/// ---@param age? number
/// ---@return string                -- Return types
/// ---@return number?
/// ---@overload fun(name: string): string  -- Function overloads
/// function getName(name, age) end
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct FunctionSignature {
    pub params: Vec<FunctionParam>,
    pub returns: Vec<LuaType>,
    pub async_fn: bool,
    pub deprecated: bool,
    pub nodiscard: bool,
    pub package: bool,
    pub overloads: Vec<FunctionSignature>,
    pub generics: Vec<String>,
    pub documentation: Option<String>,
}

// Class-related definitions
/// Field visibility scope for class members.
///
/// # Examples
/// ```lua
/// ---@class MyClass
/// ---@field public_field string     -- Public field (default)
/// ---@field private _private number -- Private field
/// ---@field protected _protected boolean -- Protected field
/// ---@field package _package table  -- Package-private field
/// ```
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldScope {
    #[serde(rename = "")]
    Public,
    Private,
    Protected,
    Package,
}

/// Represents a class field with type, visibility, and optional status.
///
/// # Examples
/// ```lua
/// ---@class Player
/// ---@field name string              -- Required public field
/// ---@field age? number              -- Optional field
/// ---@field private _id string       -- Private field
/// ---@field protected _health number -- Protected field
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct ClassField {
    pub name: String,
    pub ty: LuaType,
    pub scope: FieldScope,
    pub optional: bool,
    pub description: Option<String>,
}

// Operator-related definitions
/// Lua metamethod operators supported by the language server.
///
/// These correspond to Lua metamethods that can be overloaded in classes.
///
/// # Examples
/// ```lua
/// ---@class Vector
/// ---@operator add(Vector): Vector   -- Overload + operator
/// ---@operator unm: Vector           -- Overload unary - operator
/// ---@operator call(number): Vector  -- Overload () operator
/// ---@operator len: number           -- Overload # operator
/// ```
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LuaOperator {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    #[serde(rename = "idiv")]
    IDiv, // //
    Mod, // %
    Pow, // ^
    Unm, // unary -
    Concat, // ..
    Len, // #
    Eq,  // ==
    Lt,  // <
    Le,  // <=
    Call, // ()
    Index, // []
    #[serde(rename = "newindex")]
    NewIndex, // []=
    #[serde(rename = "tostring")]
    ToString, // tostring()
}

/// Represents an operator overload for a class.
///
/// # Examples
/// ```lua
/// ---@class Vector
/// ---@operator add(Vector): Vector   -- Binary operator with parameter
/// ---@operator unm: Vector           -- Unary operator without parameter
/// ---@operator call(...): Vector     -- Call operator with variadic parameters
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Operator {
    pub operation: LuaOperator, // e.g., "add", "unm", "call"
    pub param_type: Option<LuaType>,
    pub return_type: LuaType,
}

/// Represents a Lua class with inheritance, fields, operators, and generic support.
///
/// Classes can have inheritance, generic parameters, fields with visibility modifiers,
/// operator overloads, and comprehensive documentation.
///
/// # Examples
/// ```lua
/// ---@class Vector<T>           -- Generic class
/// ---@field x number           -- Public field
/// ---@field private _id string -- Private field
/// ---@operator add(Vector<T>): Vector<T>  -- Operator overload
///
/// ---@class Point : Vector<number>  -- Inheritance
/// ---@field z number                -- Additional field
/// ```
#[derive(Debug, Clone, Serialize, Default)]
pub struct LuaClass {
    pub name: String,
    pub exact: bool,
    pub parents: Vec<String>,
    pub fields: Vec<ClassField>,
    pub generics: Vec<String>,
    pub operators: Vec<Operator>,
    pub documentation: Option<String>,
}

// Type alias and enum definitions
/// Represents an enum variant for type aliases that act as enums.
///
/// # Examples
/// ```lua
/// ---@alias Color
/// ---| "red"   -- Red color
/// ---| "green" -- Green color
/// ---| "blue"  -- Blue color
///
/// ---@enum Direction
/// local Direction = {
///     UP = 1,    -- Move up
///     DOWN = 2,  -- Move down
///     LEFT = 3,  -- Move left
///     RIGHT = 4, -- Move right
/// }
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct EnumVariant {
    pub value: String,
    pub description: Option<String>,
}

/// Represents a type alias (including enums)
///
/// Type aliases can be simple type definitions or enum-like structures with specific values.
///
/// # Examples
/// ```lua
/// ---@alias MyString string         -- Simple alias
/// ---@alias StringOrNumber string | number  -- Union alias
///
/// ---@alias Color
/// ---| "red"
/// ---| "green"
/// ---| "blue"
///
/// ---@enum Status
/// local Status = {
///     PENDING = 0,
///     SUCCESS = 1,
///     ERROR = 2
/// }
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct TypeAlias {
    pub name: String,
    pub definition: LuaType,
    pub enum_variants: Vec<EnumVariant>, // For enum-like aliases
    pub description: Option<String>,
}

// Module and file-level definitions
/// Represents a module or meta file
///
/// A module contains classes, type aliases, functions, enums, and other declarations.
/// Can be marked as a meta file for special handling by the language server.
///
/// # Examples
/// ```lua
/// ---@meta
/// ---@module "mymodule"
///
/// ---@class MyClass
/// local MyClass = {}
///
/// ---@type string
/// local version = "1.0.0"
///
/// return MyClass
/// ```
#[derive(Debug, Clone, Serialize, Default)]
pub struct LuaModule {
    pub name: String,
    pub classes: Vec<LuaClass>,
    pub aliases: Vec<TypeAlias>,
    pub functions: Vec<FunctionSignature>,
    pub enums: Vec<TypeAlias>,
    pub documentation: Option<String>,
    pub is_meta: bool,
}

/// Represents a Lua definition file
///
/// Contains all modules and diagnostic settings for a complete Lua type definition file.
/// Used to generate `.lua` files with proper annotations for the Lua Language Server.
///
/// # Examples
/// ```lua
/// ---@meta
/// ---@diagnostic disable: lowercase-global
///
/// ---@class MyClass
/// local MyClass = {}
///
/// return MyClass
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct LuaDefinitionFile {
    pub modules: Vec<LuaModule>,
    pub diagnostics: Vec<DiagnosticToggle>,
}

// Diagnostic-related definitions
/// Represents a diagnostic toggle annotation.
///
/// Used to control which diagnostics are enabled or disabled for specific scopes.
///
/// # Examples
/// ```lua
/// ---@diagnostic disable: lowercase-global        -- Disable for entire file
/// ---@diagnostic disable-next-line: unused-local  -- Disable for next line only
/// ---@diagnostic enable: undefined-global         -- Re-enable specific diagnostic
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticToggle {
    pub state: DiagnosticState,
    pub diagnostics: Vec<String>,
    pub scope: DiagnosticScope,
}

/// Diagnostic state (enable/disable)
///
/// Controls whether diagnostics are enabled, disabled, or disabled for specific lines.
///
/// # Examples
/// ```lua
/// ---@diagnostic enable: undefined-global         -- Enable diagnostic
/// ---@diagnostic disable: lowercase-global        -- Disable diagnostic  
/// ---@diagnostic disable-next-line: unused-local  -- Disable for next line
/// ---@diagnostic disable-line: missing-parameter  -- Disable for current line
/// ```
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticState {
    Enable,
    Disable,
    #[serde(rename = "disable-next-line")]
    DisableNextLine,
    #[serde(rename = "disable-line")]
    DisableLine,
}

/// Where the diagnostic toggle applies
///
/// Defines the scope where diagnostic settings take effect.
///
/// # Examples
/// ```lua
/// ---@diagnostic disable: lowercase-global    -- File scope
/// local x = 1
/// ---@diagnostic disable-next-line: unused-local  -- Next line scope
/// local unused = 2
/// ```
#[derive(Debug, Clone, Serialize)]
pub enum DiagnosticScope {
    File,
    Line(usize),
    NextLine(usize),
}
