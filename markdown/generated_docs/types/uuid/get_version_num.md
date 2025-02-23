# get\_version\_num

>  Returns the version number of the UUID.
>  This represents the algorithm used to generate the value.
>  This method is the future-proof alternative to [`Uuid::get_version`].
>  # Examples
>  Basic usage:
>  ```
>  # use uuid::Uuid;
>  # fn main() -> Result<(), uuid::Error> {
>  let my_uuid = Uuid::parse_str("02f09a3f-1624-3b1d-8409-44eff7708208")?;
>  assert_eq!(3, my_uuid.get_version_num());
>  # Ok(())
>  # }
>  ```
>  # References
>  * [Version Field in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-4.2)

#### Arguments

- **\_self** : `Uuid` \- No Documentation 🚧

#### Returns

- **arg0** : `usize` \- No Documentation 🚧