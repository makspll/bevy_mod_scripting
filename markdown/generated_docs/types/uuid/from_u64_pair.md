# from\_u64\_pair

>  Creates a UUID from two 64bit values.
>  # Examples
>  Basic usage:
>  ```
>  # use uuid::Uuid;
>  let hi = 0xa1a2a3a4b1b2c1c2u64;
>  let lo = 0xd1d2d3d4d5d6d7d8u64;
>  let uuid = Uuid::from_u64_pair(hi, lo);
>  assert_eq!(
>      "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8",
>      uuid.hyphenated().to_string(),
>  );
>  ```

#### Arguments

- **high\_bits** : `u64` \- No Documentation 🚧
- **low\_bits** : `u64` \- No Documentation 🚧

#### Returns

- **arg0** : `Uuid` \- No Documentation 🚧