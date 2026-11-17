# Builtins

<div class="lad-page"></div>

The only globals of scripts. Everything else is imported from a namespace, see the namespace pages under Types, i.e. `import_namespace("bms").world`.

## Functions

| Name | Summary |
| --- | --- |
| [`hello_world`](#hello_world) |  |
| [`typed_arguments`](#typed_arguments) |  |

### `hello_world` {#hello_world .lad-function}

<div class="lad-signature">

**hello\_world**(arg1: [usize](./types/usize.md), rest: ...) -> [usize](./types/usize.md)

</div>

### `typed_arguments` {#typed_arguments .lad-function}

<div class="lad-signature">

**typed\_arguments**\<A\>(options: Fields\<[OptionsStruct](./types/optionsstruct.md)\>, name: "first" \| "second_variant", value: A) -> [OpaqueGenericType](./types/opaquegenerictypet.md)\<A\>

</div>

<p class="lad-note">Generic over <code>A</code>.</p>

