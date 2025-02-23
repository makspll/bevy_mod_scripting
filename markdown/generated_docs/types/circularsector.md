# CircularSector

### CircularSector

- **arc** : bevy\_math::primitives::dim2::Arc2d

## Description

>  A primitive representing a circular sector: a pie slice of a circle.
> 
>  The segment is positioned so that it always includes [`Vec2::Y`] and is vertically symmetrical.
>  To orient the sector differently, apply a rotation.
>  The sector is drawn with the center of its circle at the origin [`Vec2::ZERO`].
> 
>  **Warning:** Circular sectors with negative angle or radius, or with angle greater than an entire circle, are not officially supported.
>  We recommend normalizing circular sectors to have an angle in [0, 2π].

## Functions

| Function | Summary |
| --- | --- |
| `angle(_self)` | [ Get the angle of the sector](./circularsector/angle.md) |
| `apothem(_self)` | [ Get the length of the apothem of this sector  See \[\`Arc2d::apothem\`\]](./circularsector/apothem.md) |
| `arc_length(_self)` | [ Get the length of the arc defining the sector](./circularsector/arc_length.md) |
| `chord_length(_self)` | [ Get the length of the chord defined by the sector  See \[\`Arc2d::chord\_length\`\]](./circularsector/chord_length.md) |
| `chord_midpoint(_self)` | [ Get the midpoint of the chord defined by the sector  See \[\`Arc2d::chord\_midpoint\`\]](./circularsector/chord_midpoint.md) |
| `clone(_self)` | [No Documentation 🚧](./circularsector/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./circularsector/eq.md) |
| `from_degrees(radius, angle)` | [ Create a new \[\`CircularSector\`\] from a \`radius\` and an \`angle\` in degrees\.](./circularsector/from_degrees.md) |
| `from_radians(radius, angle)` | [ Create a new \[\`CircularSector\`\] from a \`radius\` and an \`angle\` in radians\.](./circularsector/from_radians.md) |
| `from_turns(radius, fraction)` | [ Create a new \[\`CircularSector\`\] from a \`radius\` and a number of \`turns\` of a circle\.  For instance, \`0\.5\` turns is a semicircle\.](./circularsector/from_turns.md) |
| `half_angle(_self)` | [ Get half the angle of the sector](./circularsector/half_angle.md) |
| `half_chord_length(_self)` | [ Get half the length of the chord defined by the sector  See \[\`Arc2d::half\_chord\_length\`\]](./circularsector/half_chord_length.md) |
| `new(radius, angle)` | [ Create a new \[\`CircularSector\`\] from a \`radius\` and an \`angle\`](./circularsector/new.md) |
| `radius(_self)` | [ Get the radius of the sector](./circularsector/radius.md) |
| `sagitta(_self)` | [ Get the length of the sagitta of this sector  See \[\`Arc2d::sagitta\`\]](./circularsector/sagitta.md) |