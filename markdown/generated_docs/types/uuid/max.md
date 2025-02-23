# max

>  The 'max UUID' (all ones).
>  The max UUID is a special form of UUID that is specified to have all
>  128 bits set to one.
>  # References
>  * [Max UUID in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-5.10)
>  # Examples
>  Basic usage:
>  ```
>  # use uuid::Uuid;
>  let uuid = Uuid::max();
>  assert_eq!(
>      "ffffffff-ffff-ffff-ffff-ffffffffffff",
>      uuid.hyphenated().to_string(),
>  );
>  ```

#### Arguments



#### Returns

- **arg0** : `Uuid` \- No Documentation 🚧