# into\_inner

>  Consumes the atomic and returns the contained value.
>  This is safe because passing `self` by value guarantees that no other threads are
>  concurrently accessing the atomic data.
>  # Examples
>  ```
> use std::sync::atomic::AtomicUsize;
> let some_var = AtomicUsize::new(5);
>  assert_eq!(some_var.into_inner(), 5);
>  ```

#### Arguments

- **\_self** : `AtomicUsize` \- No Documentation 🚧

#### Returns

- **arg0** : `usize` \- No Documentation 🚧