# into\_inner

>  Consumes the atomic and returns the contained value.
>  This is safe because passing `self` by value guarantees that no other threads are
>  concurrently accessing the atomic data.
>  # Examples
>  ```
> use std::sync::atomic::AtomicU8;
> let some_var = AtomicU8::new(5);
>  assert_eq!(some_var.into_inner(), 5);
>  ```

#### Arguments

- **\_self** : `AtomicU8` \- No Documentation 🚧

#### Returns

- **arg0** : `u8` \- No Documentation 🚧