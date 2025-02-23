# to\_u128\_le

>  Returns a 128bit little-endian value containing the value.
>  The bytes in the `u128` will be flipped to convert into big-endian
>  order. This is based on the endianness of the UUID, rather than the
>  target environment so bytes will be flipped on both big and little
>  endian machines.
>  Note that this will produce a different result than
>  [`Uuid::to_fields_le`], because the entire UUID is reversed, rather
>  than reversing the individual fields in-place.
>  # Examples
>  ```
>  # use uuid::Uuid;
>  # fn main() -> Result<(), uuid::Error> {
>  let uuid = Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")?;
>  assert_eq!(
>      uuid.to_u128_le(),
>      0xd8d7d6d5d4d3d2d1c2c1b2b1a4a3a2a1,
>  );
>  # Ok(())
>  # }
>  ```

#### Arguments

- **\_self** : `Uuid` \- No Documentation 🚧

#### Returns

- **arg0** : `u128` \- No Documentation 🚧