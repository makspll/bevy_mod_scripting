# into\_inner

>  Consumes the atomic and returns the contained value.
>  This is safe because passing `self` by value guarantees that no other threads are
>  concurrently accessing the atomic data.
>  # Examples
>  ```
> use std::sync::atomic::AtomicI8;
> let some_var = AtomicI8::new(5);
>  assert_eq!(some_var.into_inner(), 5);
>  ```

#### Arguments

- **\_self** : `AtomicI8` \- No Documentation 🚧

#### Returns

- **arg0** : `i8` \- No Documentation 🚧