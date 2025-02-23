# from\_bytes\_le

>  Creates a UUID using the supplied bytes in little endian order.
>  The individual fields encoded in the buffer will be flipped.
>  # Examples
>  Basic usage:
>  ```
>  # fn main() -> Result<(), uuid::Error> {
>  # use uuid::Uuid;
>  let bytes = [
>      0xa1, 0xa2, 0xa3, 0xa4,
>      0xb1, 0xb2,
>      0xc1, 0xc2,
>      0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
>  ];
>  let uuid = Uuid::from_bytes_le(bytes);
>  assert_eq!(
>      "a4a3a2a1-b2b1-c2c1-d1d2-d3d4d5d6d7d8",
>      uuid.hyphenated().to_string(),
>  );
>  # Ok(())
>  # }
>  ```

#### Arguments

- **b** : `[u8; 16]` \- No Documentation 🚧

#### Returns

- **arg0** : `Uuid` \- No Documentation 🚧