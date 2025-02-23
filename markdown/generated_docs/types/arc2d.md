# Arc2d

### Arc2d

- **radius** : f32
- **half\_angle** : f32

## Description

>  A primitive representing an arc between two points on a circle.
> 
>  An arc has no area.
>  If you want to include the portion of a circle's area swept out by the arc,
>  use the pie-shaped [`CircularSector`].
>  If you want to include only the space inside the convex hull of the arc,
>  use the bowl-shaped [`CircularSegment`].
> 
>  The arc is drawn starting from [`Vec2::Y`], extending by `half_angle` radians on
>  either side. The center of the circle is the origin [`Vec2::ZERO`]. Note that this
>  means that the origin may not be within the `Arc2d`'s convex hull.
> 
>  **Warning:** Arcs with negative angle or radius, or with angle greater than an entire circle, are not officially supported.
>  It is recommended to normalize arcs to have an angle in [0, 2π].

## Functions

| Function | Summary |
| --- | --- |
| `angle(_self)` | [ Get the angle of the arc](./arc2d/angle.md) |
| `apothem(_self)` | [ Get the length of the apothem of this arc, that is,  the distance from the center of the circle to ](./arc2d/apothem.md) |
| `chord_length(_self)` | [ Get the distance between the endpoints \(the length of the chord\)](./arc2d/chord_length.md) |
| `chord_midpoint(_self)` | [ Get the midpoint of the two endpoints \(the midpoint of the chord\)](./arc2d/chord_midpoint.md) |
| `clone(_self)` | [No Documentation 🚧](./arc2d/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./arc2d/eq.md) |
| `from_degrees(radius, angle)` | [ Create a new \[\`Arc2d\`\] from a \`radius\` and an \`angle\` in degrees\.](./arc2d/from_degrees.md) |
| `from_radians(radius, angle)` | [ Create a new \[\`Arc2d\`\] from a \`radius\` and an \`angle\` in radians](./arc2d/from_radians.md) |
| `from_turns(radius, fraction)` | [ Create a new \[\`Arc2d\`\] from a \`radius\` and a \`fraction\` of a single turn\.  For instance, \`0\.5\` turns is a semicircle\.](./arc2d/from_turns.md) |
| `half_chord_length(_self)` | [ Get half the distance between the endpoints \(half the length of the chord\)](./arc2d/half_chord_length.md) |
| `is_major(_self)` | [ Produces true if the arc is at least half a circle\.  \*\*Note:\*\* This is not the negation of \[\`is\_minor\`\]\(Self::is\_minor\): an exact semicircle is both major and minor\.](./arc2d/is_major.md) |
| `is_minor(_self)` | [ Produces true if the arc is at most half a circle\.  \*\*Note:\*\* This is not the negation of \[\`is\_major\`\]\(Self::is\_major\): an exact semicircle is both major and minor\.](./arc2d/is_minor.md) |
| `left_endpoint(_self)` | [ Get the left\-hand end point of the arc](./arc2d/left_endpoint.md) |
| `length(_self)` | [ Get the length of the arc](./arc2d/length.md) |
| `midpoint(_self)` | [ Get the midpoint of the arc](./arc2d/midpoint.md) |
| `new(radius, half_angle)` | [ Create a new \[\`Arc2d\`\] from a \`radius\` and a \`half\_angle\`](./arc2d/new.md) |
| `right_endpoint(_self)` | [ Get the right\-hand end point of the arc](./arc2d/right_endpoint.md) |
| `sagitta(_self)` | [ Get the length of the sagitta of this arc, that is,  the length of the line between the midpoints o](./arc2d/sagitta.md) |