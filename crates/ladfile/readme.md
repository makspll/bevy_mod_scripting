# Language Agnostic Declaration file

A file format specifying the available exported:
- functions
- types
- primitives
- documentation

For a `bevy` game engine project.

## Example

```json
{
  "version": "0.1.0",
  "types": {
    "ladfile::test::EnumType": {
      "identifier": "EnumType",
      "crate": "ladfile",
      "path": "ladfile::test::EnumType",
      "layout": [
        {
          "kind": "Unit",
          "name": "Unit"
        },
        {
          "kind": "Struct",
          "name": "Struct",
          "fields": [
            {
              "name": "field",
              "type": "usize"
            }
          ]
        },
        {
          "kind": "TupleStruct",
          "name": "TupleStruct",
          "fields": [
            {
              "type": "usize"
            },
            {
              "type": "alloc::string::String"
            }
          ]
        }
      ]
    },
    "ladfile::test::StructType<usize>": {
      "identifier": "StructType",
      "crate": "ladfile",
      "path": "ladfile::test::StructType<usize>",
      "generics": [
        {
          "type_id": "usize",
          "name": "T"
        }
      ],
      "documentation": " I am a struct",
      "layout": {
        "kind": "Struct",
        "name": "StructType",
        "fields": [
          {
            "name": "field",
            "type": "usize"
          },
          {
            "name": "field2",
            "type": "usize"
          }
        ]
      }
    },
    "ladfile::test::TupleStructType": {
      "identifier": "TupleStructType",
      "crate": "ladfile",
      "path": "ladfile::test::TupleStructType",
      "documentation": " I am a tuple test type",
      "layout": {
        "kind": "TupleStruct",
        "name": "TupleStructType",
        "fields": [
          {
            "type": "usize"
          },
          {
            "type": "alloc::string::String"
          }
        ]
      }
    },
    "ladfile::test::UnitType": {
      "identifier": "UnitType",
      "crate": "ladfile",
      "path": "ladfile::test::UnitType",
      "documentation": " I am a unit test type",
      "layout": {
        "kind": "Struct",
        "name": "UnitType"
      }
    }
  },
  "functions": {
    "::hello_world": {
      "identifier": "hello_world",
      "arguments": [
        {
          "kind": {
            "primitive": "usize"
          },
          "name": "arg1"
        }
      ],
      "return_type": "usize"
    },
    "ladfile::test::StructType<usize>::hello_world": {
      "identifier": "hello_world",
      "arguments": [
        {
          "kind": {
            "primitive": "reflectReference"
          },
          "name": "ref_"
        },
        {
          "kind": {
            "tuple": [
              {
                "primitive": "usize"
              },
              {
                "primitive": "string"
              }
            ]
          },
          "name": "tuple"
        },
        {
          "kind": {
            "option": {
              "vec": {
                "ref": "ladfile::test::EnumType"
              }
            }
          },
          "name": "option_vec_ref_wrapper"
        }
      ],
      "return_type": "usize"
    }
  },
  "primitives": {
    "TypeId(0x0b36ea25c1cf517efce182c726ea2190)": {
      "kind": "pathBuf",
      "documentation": "A heap allocated file path"
    },
    "TypeId(0x1c306727557831f62320b5841ddc7eb3)": {
      "kind": "dynamicFunction",
      "documentation": "A callable dynamic function"
    },
    "TypeId(0x7adbf8cf2ed263727e95f06e821c8654)": {
      "kind": "osString",
      "documentation": "A heap allocated OS string"
    },
    "TypeId(0x7f945ad2d333d63863e3b6f35dfc0c5d)": {
      "kind": "dynamicFunctionMut",
      "documentation": "A stateful and callable dynamic function"
    },
    "TypeId(0xb98b1b7157a6417863eb502cd6cb5d6d)": {
      "kind": "str",
      "documentation": "A static string slice"
    },
    "alloc::string::String": {
      "kind": "string",
      "documentation": "A heap allocated string"
    },
    "bevy_mod_scripting_core::bindings::function::script_function::FunctionCallContext": {
      "kind": "functionCallContext",
      "documentation": "Function call context, if accepted by a function, means the function can access the world in arbitrary ways."
    },
    "bevy_mod_scripting_core::bindings::reference::ReflectReference": {
      "kind": "reflectReference",
      "documentation": "A reference to a reflectable type"
    },
    "bool": {
      "kind": "bool",
      "documentation": "A boolean value"
    },
    "char": {
      "kind": "char",
      "documentation": "An 8-bit character"
    },
    "f32": {
      "kind": "f32",
      "documentation": "A 32-bit floating point number"
    },
    "f64": {
      "kind": "f64",
      "documentation": "A 64-bit floating point number"
    },
    "i128": {
      "kind": "i128",
      "documentation": "A signed 128-bit integer"
    },
    "i16": {
      "kind": "i16",
      "documentation": "A signed 16-bit integer"
    },
    "i32": {
      "kind": "i32",
      "documentation": "A signed 32-bit integer"
    },
    "i64": {
      "kind": "i64",
      "documentation": "A signed 64-bit integer"
    },
    "i8": {
      "kind": "i8",
      "documentation": "A signed 8-bit integer"
    },
    "isize": {
      "kind": "isize",
      "documentation": "A signed pointer-sized integer"
    },
    "u128": {
      "kind": "u128",
      "documentation": "An unsigned 128-bit integer"
    },
    "u16": {
      "kind": "u16",
      "documentation": "An unsigned 16-bit integer"
    },
    "u32": {
      "kind": "u32",
      "documentation": "An unsigned 32-bit integer"
    },
    "u64": {
      "kind": "u64",
      "documentation": "An unsigned 64-bit integer"
    },
    "u8": {
      "kind": "u8",
      "documentation": "An unsigned 8-bit integer"
    },
    "usize": {
      "kind": "usize",
      "documentation": "An unsigned pointer-sized integer"
    }
  }
}
```
