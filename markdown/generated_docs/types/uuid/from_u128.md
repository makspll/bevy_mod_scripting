# from\_u128

>  Creates a UUID from a 128bit value.
>  # Examples
>  Basic usage:
>  ```
>  # use uuid::Uuid;
>  let v = 0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128;
>  let uuid = Uuid::from_u128(v);
>  assert_eq!(
>      "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8",
>      uuid.hyphenated().to_string(),
>  );
>  ```

#### Arguments

- **v** : `u128` \- No Documentation 🚧

#### Returns

- **arg0** : `Uuid` \- No Documentation 🚧