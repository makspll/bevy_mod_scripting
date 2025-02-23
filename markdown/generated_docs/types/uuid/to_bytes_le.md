# to\_bytes\_le

>  Returns the bytes of the UUID in little-endian order.
>  The bytes will be flipped to convert into little-endian order. This is
>  based on the endianness of the UUID, rather than the target environment
>  so bytes will be flipped on both big and little endian machines.
>  # Examples
>  ```
>  use uuid::Uuid;
>  # fn main() -> Result<(), uuid::Error> {
>  let uuid = Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")?;
>  assert_eq!(
>      uuid.to_bytes_le(),
>      ([
>          0xa4, 0xa3, 0xa2, 0xa1, 0xb2, 0xb1, 0xc2, 0xc1, 0xd1, 0xd2,
>          0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8
>      ])
>  );
>  # Ok(())
>  # }
>  ```

#### Arguments

- **\_self** : `Uuid` \- No Documentation 🚧

#### Returns

- **arg0** : `[u8; 16]` \- No Documentation 🚧