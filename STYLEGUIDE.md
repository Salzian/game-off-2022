# Styleguide

## Systems

### Function parameters

#### Sorting

1. Sort by category
   1. `Commands`
   2. `Res<_>`
   3. `ResMut<_>`
2. Sort alphabetically

#### Example

- Res first, then MutRes
  ```rust
  fn my_system(
    commands: Commands,
    abc: Res<AbcStuff>,
    xyz: Res<XyzStuff>,
    mut abcd: ResMut<AbcdStuff>,
    mut xyza: ResMut<XyzaStuff>,
  ) {}
  ```
