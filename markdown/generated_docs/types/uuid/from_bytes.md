# from\_bytes

>  Creates a UUID using the supplied bytes.
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
>  let uuid = Uuid::from_bytes(bytes);
>  assert_eq!(
>      uuid.hyphenated().to_string(),
>      "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"
>  );
>  # Ok(())
>  # }
>  ```

#### Arguments

- **bytes** : `[u8; 16]` \- No Documentation 🚧

#### Returns

- **arg0** : `Uuid` \- No Documentation 🚧