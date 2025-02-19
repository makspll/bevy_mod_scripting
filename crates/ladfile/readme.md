# Language Agnostic Declaration file

A file format specifying the available exported:
- functions
- types
- primitives
- documentation

For a `bevy` game engine project.

## Example
See an example of a `LAD` file [here](./test_assets/test.lad.json)

## Features

- `testfile` - Include the above testfile as a `ladfile::EXAMPLE_LADFILE` constant
- `visitor` - Provide traits for visiting parts of the `LAD` file.