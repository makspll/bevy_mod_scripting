# into\_inner

>  Consumes the atomic and returns the contained value.
>  This is safe because passing `self` by value guarantees that no other threads are
>  concurrently accessing the atomic data.
>  # Examples
>  ```
>  use std::sync::atomic::AtomicBool;
>  let some_bool = AtomicBool::new(true);
>  assert_eq!(some_bool.into_inner(), true);
>  ```

#### Arguments

- **\_self** : `AtomicBool` \- No Documentation 🚧

#### Returns

- **arg0** : `bool` \- No Documentation 🚧