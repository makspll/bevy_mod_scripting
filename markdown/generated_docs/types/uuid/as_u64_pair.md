# as\_u64\_pair

>  Returns two 64bit values containing the value.
>  The bytes in the UUID will be split into two `u64`.
>  The first u64 represents the 64 most significant bits,
>  the second one represents the 64 least significant.
>  # Examples
>  ```
>  # use uuid::Uuid;
>  # fn main() -> Result<(), uuid::Error> {
>  let uuid = Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")?;
>  assert_eq!(
>      uuid.as_u64_pair(),
>      (0xa1a2a3a4b1b2c1c2, 0xd1d2d3d4d5d6d7d8),
>  );
>  # Ok(())
>  # }
>  ```

#### Arguments

- **\_self** : `Uuid` \- No Documentation 🚧

#### Returns

- **arg0** : `(u64, u64)` \- No Documentation 🚧