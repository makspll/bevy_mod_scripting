# Uuid

Opaque Type\. 🔒

## Description

> No Documentation 🚧

## Functions

| Function | Summary |
| --- | --- |
| `as_u128(_self)` | [ Returns a 128bit value containing the value\.  The bytes in the UUID will be packed directly into a \`u128\`](./uuid/as_u128.md) |
| `as_u64_pair(_self)` | [ Returns two 64bit values containing the value\.  The bytes in the UUID will be split into two \`u64\`\.](./uuid/as_u64_pair.md) |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./uuid/assert_receiver_is_total_eq.md) |
| `clone(_self)` | [No Documentation 🚧](./uuid/clone.md) |
| `encode_buffer()` | [ A buffer that can be used for \`encode\_\.\.\.\` calls, that is  guaranteed to be long enough for any of the format adapters\.  \# Examples  \`\`\`  \# use uuid::Uuid;  let uuid = Uuid::nil\(\);  assert\_](./uuid/encode_buffer.md) |
| `eq(_self, other)` | [No Documentation 🚧](./uuid/eq.md) |
| `from_bytes(bytes)` | [ Creates a UUID using the supplied bytes\.  \# Examples  Basic usage:  \`\`\`  \# fn main\(\) \-> Result<\(\), uuid::Error> \{  \# use uuid::Uuid;  let bytes = \[      0xa1, 0xa2, 0xa3, 0xa4,      0xb1, 0xb2,      0xc1, 0xc2,      0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,  \];  let uuid = Uuid::from\_bytes\(bytes\);  assert\_eq\!\(      uuid\.hyphenated\(\)\.to\_string\(\),      "a1a2a3a4\-b1b2\-c1c2\-d1d2\-d3d4d5d6d7d8"  \);  \# Ok\(\(\)\)  \# \}  \`\`\`](./uuid/from_bytes.md) |
| `from_bytes_le(b)` | [ Creates a UUID using the supplied bytes in little endian order\.  The individual fields encoded in t](./uuid/from_bytes_le.md) |
| `from_u128(v)` | [ Creates a UUID from a 128bit value\.  \# Examples  Basic usage:  \`\`\`  \# use uuid::Uuid;  let v = 0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128;  let uuid = Uuid::from\_u128\(v\);  assert\_eq\!\(      "a1a2a3a4\-b1b2\-c1c2\-d1d2\-d3d4d5d6d7d8",      uuid\.hyphenated\(\)\.to\_string\(\),  \);  \`\`\`](./uuid/from_u128.md) |
| `from_u128_le(v)` | [ Creates a UUID from a 128bit value in little\-endian order\.  The entire value will be flipped to con](./uuid/from_u128_le.md) |
| `from_u64_pair(high_bits, low_bits)` | [ Creates a UUID from two 64bit values\.  \# Examples  Basic usage:  \`\`\`  \# use uuid::Uuid;  let hi = 0xa1a2a3a4b1b2c1c2u64;  let lo = 0xd1d2d3d4d5d6d7d8u64;  let uuid = Uuid::from\_u64\_pair\(hi, lo\);  assert\_eq\!\(      "a1a2a3a4\-b1b2\-c1c2\-d1d2\-d3d4d5d6d7d8",      uuid\.hyphenated\(\)\.to\_string\(\),  \);  \`](./uuid/from_u64_pair.md) |
| `get_node_id(_self)` | [ If the UUID is the correct version \(v1, or v6\) this will return the  node value as a 6\-byte array\. ](./uuid/get_node_id.md) |
| `get_version_num(_self)` | [ Returns the version number of the UUID\.  This represents the algorithm used to generate the value\. ](./uuid/get_version_num.md) |
| `into_bytes(_self)` | [ Consumes self and returns the underlying byte value of the UUID\.  \# Examples  \`\`\`  \# use uuid::Uuid;  let bytes = \[      0xa1, 0xa2, 0xa3, 0xa4,      0xb1, 0xb2,      0xc1, 0xc2,      0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,  \];  let uuid = Uuid::from\_bytes\(bytes\);  assert\_eq\!\(bytes, uuid\.into\_bytes\(\)\);  \`\`\`](./uuid/into_bytes.md) |
| `is_max(_self)` | [ Tests if the UUID is max \(all ones\)\.](./uuid/is_max.md) |
| `is_nil(_self)` | [ Tests if the UUID is nil \(all zeros\)\.](./uuid/is_nil.md) |
| `max()` | [ The 'max UUID' \(all ones\)\.  The max UUID is a special form of UUID that is specified to have all  1](./uuid/max.md) |
| `new_v4()` | [ Creates a random UUID\.  This uses the \[\`getrandom\`\] crate to utilise the operating system's RNG  as the source of random numbers\. If you'd like to use a custom  generator, don't use this method: generate random bytes using your  custom generator and pass them to the  \[\`uuid::Builder::from\_random\_bytes\`\]](./uuid/new_v4.md) |
| `nil()` | [ The 'nil UUID' \(all zeros\)\.  The nil UUID is a special form of UUID that is specified to have all  ](./uuid/nil.md) |
| `to_bytes_le(_self)` | [ Returns the bytes of the UUID in little\-endian order\.  The bytes will be flipped to convert into li](./uuid/to_bytes_le.md) |
| `to_u128_le(_self)` | [ Returns a 128bit little\-endian value containing the value\.  The bytes in the \`u128\` will be flipped](./uuid/to_u128_le.md) |