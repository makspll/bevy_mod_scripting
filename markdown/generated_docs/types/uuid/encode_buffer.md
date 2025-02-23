# encode\_buffer

>  A buffer that can be used for `encode_...` calls, that is
>  guaranteed to be long enough for any of the format adapters.
>  # Examples
>  ```
>  # use uuid::Uuid;
>  let uuid = Uuid::nil();
>  assert_eq!(
>      uuid.simple().encode_lower(&mut Uuid::encode_buffer()),
>      "00000000000000000000000000000000"
>  );
>  assert_eq!(
>      uuid.hyphenated()
>          .encode_lower(&mut Uuid::encode_buffer()),
>      "00000000-0000-0000-0000-000000000000"
>  );
>  assert_eq!(
>      uuid.urn().encode_lower(&mut Uuid::encode_buffer()),
>      "urn:uuid:00000000-0000-0000-0000-000000000000"
>  );
>  ```

#### Arguments



#### Returns

- **arg0** : `[u8; 45]` \- No Documentation 🚧