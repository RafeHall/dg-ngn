#import "@preview/cheq:0.2.0": checklist
#import "@preview/fletcher:0.5.1" as fletcher: diagram, node, edge

#show: checklist

- [x] Vec2
- [ ] IVec2

- [x] Vec3
- [ ] IVec3

- [x] Vec4
- [ ] IVec4

- [/] Matrix3
- [/] Matrix4

- [ ] Rotor / Quaternion

- [/] Transform2D
- [/] Transform3D

- [ ] Frustrum

- [/] Rect
- [ ] IRect

- [ ] AABB

- [/] Line

- [/] Plane

- [/] Interpolation

- [/] Color

#pagebreak()

#let x() = text(fill: red, [x])
#let y() = text(fill: green, [y])
#let z() = text(fill: blue, [z])

#align(center)[
  #table(
    columns: (auto, auto, auto, auto, auto),
    inset: 6pt,
    align: horizon,
    table.header(
      [], [Hand], [World], [Camera], [Unit]
    ),
    [Unity], [Left], [(+#x(), +#y(), +#z())], [(+#x(), +#y(), +#z())], [m],
    [Unreal], [Left], [(+#y(), +#z(), +#x())], [(+#y(), +#z(), +#x())], [cm],
    [Source], [Right], [(-#y(), +#z(), +#x())], [(-#y(), +#z(), +#x())], [3/4 inch],
    [Godot], [Right], [(-#x(), +#y(), +#z())], [(+#x(), +#y(), -#z())], [m],
    [Blender], [Right], [(+#x(), +#z(), +#y())], [(+#x(), +#y(), -#z())], [none],
    [Maya], [Right], [(-#x(), +#y(), +#z())], [(+#x(), +#y(), -#z())], [cm],
    [gltf], [Right], [(#sym.plus.minus?, +#y()/#z, #sym.plus.minus?)], [(+#x(), +#y(), -#z())], [m],
    [usd], [Right], [(#sym.plus.minus?, +#y()/#z, #sym.plus.minus?)], [(+#x(), +#y(), -#z())], [cm],
    [*dg-ngn*], [Right], [(+#x(), +#y(), -#z())], [(+#x(), +#y(), -#z())], [m],
  )
]

#table(
  columns: (1fr, 1fr),
  gutter: 3pt,
  align: center + horizon,
  inset: 8pt,
  [*2D*], [*3D*],
  table.cell()[
    #set text(size: 8pt)
    #let (norigin, nx, ny, nnx, nny) = ((0, 0), (1, 0), (0, 1), (-0.5, 0), (0, -0.5));
    #diagram(
      spacing: 7em,
      node(norigin, [(0, 0)]),
      node(nx, [(1, 0)]),
      node(ny, [(0, 1)]),
      node(nnx, [(-1, 0)]),
      node(nny, [(0, -1)]),

      edge(norigin, "->", nx, [+#x()]),
      edge(norigin, nx, stroke: none, label-size: 6pt, label-angle: auto, label-pos: 0.5, label-side: right, [Right]),
      edge(norigin, "->", ny, [+#y()]),
      edge(norigin, ny, stroke: none, label-size: 6pt, label-side: left, [Down]),

      edge(norigin, "->", nnx, [-#x()]),
      edge(norigin, nnx, stroke: none, label-size: 6pt, label-side: left, [Left]),
      edge(norigin, "->", nny, label-side: left, [-#y()]),
      edge(norigin, nny, stroke: none, label-size: 6pt, label-side: right, [Up]),
    )
  ],
  table.cell()[
    #set text(size: 8pt)
    #let (norigin, nx, ny, nz, nnx, nny, nnz) = ((0, 0), (1, 0), (0, 1), (0.707 * 0.75, 0.707 * 0.75), (-0.75, 0), (0, -0.75), (0.707 * -0.5, 0.707 * -0.5));
    #diagram(
      spacing: 7em,
      axes: (ltr, btt),

      node(norigin, [(0, 0, 0)]),
      node(nx, [(1, 0, 0)]),
      node(ny, [(0, 1, 0)]),
      node(nz, [(0, 0, -1)]),
      node(nnx, [(-1, 0, 0)]),
      node(nny, [(0, -1, 0)]),
      node(nnz, [(0, 0, 1)]),

      edge(norigin, "->", nx, label-side: left, [+#x()]),
      edge(norigin, nx, stroke: none, label-size: 6pt, label-pos: 0.5, label-side: right, [Right]),
      edge(norigin, "->", ny, label-side: left, [+#y()]),
      edge(norigin, ny, stroke: none, label-size: 6pt, label-pos: 0.5, label-side: right, [Up]),
      edge(norigin, "->", nz, [-#z()]),
      edge(norigin, nz, stroke: none, label-size: 6pt, label-angle: auto, label-pos: 0.5, label-side: right, [Forward]),

      edge(norigin, "->", nnx, [-#x()]),
      edge(norigin, nnx, stroke: none, label-size: 6pt, label-angle: auto, label-pos: 0.5, label-side: left, [Left]),
      edge(norigin, "->", nny, label-side: right, [-#y()]),
      edge(norigin, nny, stroke: none, label-size: 6pt, label-pos: 0.5, label-side: left, [Down]),
      edge(norigin, "->", nnz, [#z()]),
      edge(norigin, nnz, stroke: none, label-size: 6pt, label-angle: auto, label-pos: 0.5, label-side: left, [Back]),
    )
  ],
)