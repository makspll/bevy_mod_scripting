# nil

>  The 'nil UUID' (all zeros).
>  The nil UUID is a special form of UUID that is specified to have all
>  128 bits set to zero.
>  # References
>  * [Nil UUID in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-5.9)
>  # Examples
>  Basic usage:
>  ```
>  # use uuid::Uuid;
>  let uuid = Uuid::nil();
>  assert_eq!(
>      "00000000-0000-0000-0000-000000000000",
>      uuid.hyphenated().to_string(),
>  );
>  ```

#### Arguments



#### Returns

- **arg0** : `Uuid` \- No Documentation 🚧