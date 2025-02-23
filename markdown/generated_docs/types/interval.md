# Interval

### Interval

- **start** : f32
- **end** : f32

## Description

>  A nonempty closed interval, possibly unbounded in either direction.
> 
>  In other words, the interval may stretch all the way to positive or negative infinity, but it
>  will always have some nonempty interior.

## Functions

| Function | Summary |
| --- | --- |
| `clamp(_self, value)` | [ Clamp the given \`value\` to lie within this interval\.](./interval/clamp.md) |
| `clone(_self)` | [No Documentation 🚧](./interval/clone.md) |
| `contains(_self, item)` | [ Returns \`true\` if \`item\` is contained in this interval\.](./interval/contains.md) |
| `contains_interval(_self, other)` | [ Returns \`true\` if the other interval is contained in this interval\.  This is non\-strict: each inter](./interval/contains_interval.md) |
| `end(_self)` | [ Get the end of this interval\.](./interval/end.md) |
| `eq(_self, other)` | [No Documentation 🚧](./interval/eq.md) |
| `has_finite_end(_self)` | [ Returns \`true\` if this interval has a finite end\.](./interval/has_finite_end.md) |
| `has_finite_start(_self)` | [ Returns \`true\` if this interval has a finite start\.](./interval/has_finite_start.md) |
| `is_bounded(_self)` | [ Returns \`true\` if this interval is bounded — that is, if both its start and end are finite\.  Equi](./interval/is_bounded.md) |
| `length(_self)` | [ Get the length of this interval\. Note that the result may be infinite \(\`f32::INFINITY\`\)\.](./interval/length.md) |
| `start(_self)` | [ Get the start of this interval\.](./interval/start.md) |