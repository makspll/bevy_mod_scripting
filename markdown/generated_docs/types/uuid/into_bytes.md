# into\_bytes

>  Consumes self and returns the underlying byte value of the UUID.
>  # Examples
>  ```
>  # use uuid::Uuid;
>  let bytes = [
>      0xa1, 0xa2, 0xa3, 0xa4,
>      0xb1, 0xb2,
>      0xc1, 0xc2,
>      0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
>  ];
>  let uuid = Uuid::from_bytes(bytes);
>  assert_eq!(bytes, uuid.into_bytes());
>  ```

#### Arguments

- **\_self** : `Uuid` \- No Documentation 🚧

#### Returns

- **arg0** : `[u8; 16]` \- No Documentation 🚧