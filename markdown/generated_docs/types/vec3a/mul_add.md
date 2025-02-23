# mul\_add

>  Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
>  error, yielding a more accurate result than an unfused multiply-add.
>  Using `mul_add` *may* be more performant than an unfused multiply-add if the target
>  architecture has a dedicated fma CPU instruction. However, this is not always true,
>  and will be heavily dependant on designing algorithms with specific target hardware in
>  mind.

#### Arguments

- **\_self** : `Vec3A` \- No Documentation 🚧
- **a** : `Vec3A` \- No Documentation 🚧
- **b** : `Vec3A` \- No Documentation 🚧

#### Returns

- **arg0** : `Vec3A` \- No Documentation 🚧