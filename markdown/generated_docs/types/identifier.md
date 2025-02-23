# Identifier

Opaque Type\. 🔒

## Description

>  A unified identifier for all entity and similar IDs.
> 
>  Has the same size as a `u64` integer, but the layout is split between a 32-bit low
>  segment, a 31-bit high segment, and the significant bit reserved as type flags to denote
>  entity kinds.

## Functions

| Function | Summary |
| --- | --- |
| `clone(_self)` | [No Documentation 🚧](./identifier/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./identifier/eq.md) |
| `from_bits(value)` | [ Convert a \`u64\` into an \[\`Identifier\`\]\.  \# Panics  This method will likely panic if given \`u64\` values that did not come from \[\`Identifier::to\_bits\`\]](./identifier/from_bits.md) |
| `low(_self)` | [ Returns the value of the low segment of the \[\`Identifier\`\]\.](./identifier/low.md) |
| `masked_high(_self)` | [ Returns the masked value of the high segment of the \[\`Identifier\`\]\.  Does not include the flag bits\.](./identifier/masked_high.md) |
| `to_bits(_self)` | [ Convert the \[\`Identifier\`\] into a \`u64\`\.](./identifier/to_bits.md) |