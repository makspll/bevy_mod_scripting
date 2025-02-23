# CircularSegment

### CircularSegment

- **arc** : bevy\_math::primitives::dim2::Arc2d

## Description

>  A primitive representing a circular segment:
>  the area enclosed by the arc of a circle and its chord (the line between its endpoints).
> 
>  The segment is drawn starting from [`Vec2::Y`], extending equally on either side.
>  To orient the segment differently, apply a rotation.
>  The segment is drawn with the center of its circle at the origin [`Vec2::ZERO`].
>  When positioning a segment, the [`apothem`](Self::apothem) function may be particularly useful.
> 
>  **Warning:** Circular segments with negative angle or radius, or with angle greater than an entire circle, are not officially supported.
>  We recommend normalizing circular segments to have an angle in [0, 2π].

## Functions

| Function | Summary |
| --- | --- |
| `angle(_self)` | [ Get the angle of the segment](./circularsegment/angle.md) |
| `apothem(_self)` | [ Get the length of the apothem of this segment,  which is the signed distance between the segment an](./circularsegment/apothem.md) |
| `arc_length(_self)` | [ Get the length of the arc defining the segment](./circularsegment/arc_length.md) |
| `chord_length(_self)` | [ Get the length of the segment's base, also known as its chord](./circularsegment/chord_length.md) |
| `chord_midpoint(_self)` | [ Get the midpoint of the segment's base, also known as its chord](./circularsegment/chord_midpoint.md) |
| `clone(_self)` | [No Documentation 🚧](./circularsegment/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./circularsegment/eq.md) |
| `from_degrees(radius, angle)` | [ Create a new \[\`CircularSegment\`\] from a \`radius\` and an \`angle\` in degrees\.](./circularsegment/from_degrees.md) |
| `from_radians(radius, angle)` | [ Create a new \[\`CircularSegment\`\] from a \`radius\` and an \`angle\` in radians\.](./circularsegment/from_radians.md) |
| `from_turns(radius, fraction)` | [ Create a new \[\`CircularSegment\`\] from a \`radius\` and a number of \`turns\` of a circle\.  For instance, \`0\.5\` turns is a semicircle\.](./circularsegment/from_turns.md) |
| `half_angle(_self)` | [ Get the half\-angle of the segment](./circularsegment/half_angle.md) |
| `half_chord_length(_self)` | [ Get half the length of the segment's base, also known as its chord](./circularsegment/half_chord_length.md) |
| `new(radius, angle)` | [ Create a new \[\`CircularSegment\`\] from a \`radius\`, and an \`angle\`](./circularsegment/new.md) |
| `radius(_self)` | [ Get the radius of the segment](./circularsegment/radius.md) |
| `sagitta(_self)` | [ Get the length of the sagitta of this segment, also known as its height  See \[\`Arc2d::sagitta\`\]](./circularsegment/sagitta.md) |