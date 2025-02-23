# new\_v4

>  Creates a random UUID.
>  This uses the [`getrandom`] crate to utilise the operating system's RNG
>  as the source of random numbers. If you'd like to use a custom
>  generator, don't use this method: generate random bytes using your
>  custom generator and pass them to the
>  [`uuid::Builder::from_random_bytes`][from_random_bytes] function
>  instead.
>  Note that usage of this method requires the `v4` feature of this crate
>  to be enabled.
>  # Examples
>  Basic usage:
>  ```
>  # use uuid::{Uuid, Version};
>  let uuid = Uuid::new_v4();
>  assert_eq!(Some(Version::Random), uuid.get_version());
>  ```
>  # References
>  * [UUID Version 4 in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-5.4)
>  [`getrandom`]: https://crates.io/crates/getrandom
>  [from_random_bytes]: struct.Builder.html#method.from_random_bytes

#### Arguments



#### Returns

- **arg0** : `Uuid` \- No Documentation 🚧