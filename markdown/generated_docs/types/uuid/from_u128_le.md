# from\_u128\_le

>  Creates a UUID from a 128bit value in little-endian order.
>  The entire value will be flipped to convert into big-endian order.
>  This is based on the endianness of the UUID, rather than the target
>  environment so bytes will be flipped on both big and little endian
>  machines.
>  # Examples
>  Basic usage:
>  ```
>  # use uuid::Uuid;
>  let v = 0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128;
>  let uuid = Uuid::from_u128_le(v);
>  assert_eq!(
>      "d8d7d6d5-d4d3-d2d1-c2c1-b2b1a4a3a2a1",
>      uuid.hyphenated().to_string(),
>  );
>  ```

#### Arguments

- **v** : `u128` \- No Documentation 🚧

#### Returns

- **arg0** : `Uuid` \- No Documentation 🚧