# UV Mapping Documentation

## Overview

This document captures the core technical decisions, lessons learned, and implementation details for UV mapping in the Nova voxel engine. The UV mapping system evolved through multiple iterations to solve texture stretching, orientation consistency, and tiling correctness in greedy-meshed geometry.

## Core Principles

### 1. Fixed Voxel-Size Texture Tiling

**Principle**: Textures tile at a fixed world-space size determined by the smallest voxel unit (0.25m at LOD 0). Textures do NOT scale - they tile at this fixed size regardless of voxel merging or LOD level.

**Examples**:
- Single smallest voxel (0.25m): 1 texture tile
- 4 voxels side-by-side (1.0m): 4 horizontal tiles
- 4×4 grid (1.0m × 1.0m): 16 tiles (4×4)
- LOD 1 voxel (0.5m): 4 tiles (2×2)
- LOD 2 voxel (1.0m): 16 tiles (4×4)

**Implementation**:
```rust
// Calculate UVs based on quad size in voxel units
let texture_u_voxels = u_size / voxel_size;  // Number of voxels wide
let texture_v_voxels = v_size / voxel_size;  // Number of voxels tall

// UV coordinates tile once per voxel
let corner_uvs = [
    [0.0, texture_v_voxels],              // Bottom-left (V flipped)
    [texture_u_voxels, texture_v_voxels], // Bottom-right (V flipped)
    [texture_u_voxels, 0.0],              // Top-right (V flipped)
    [0.0, 0.0],                           // Top-left (V flipped)
];
```

**Rationale**: This ensures visual consistency across LOD levels and greedy-meshed geometry. A 4-voxel-wide wall looks identical whether rendered as 4 separate quads or 1 merged quad.

### 2. V-Axis Flip for Image Storage Convention

**Problem**: Image storage (row 0 = top) conflicts with UV convention (V=0 = bottom), causing textures to appear upside down.

**Solution**: Flip V coordinates in the corner_uvs array:
```rust
// NOTE: V is flipped because image row 0 is at top, but UV V=0 is at bottom
let corner_uvs = [
    [0.0, texture_v_voxels],              // Bottom-left (V flipped)
    [texture_u_voxels, texture_v_voxels], // Bottom-right (V flipped)
    [texture_u_voxels, 0.0],              // Top-right (V flipped)
    [0.0, 0.0],                           // Top-left (V flipped)
];
```

**Before**: Letter "F" appeared upside down with horizontal bar at bottom
**After**: Letter "F" appears upright with horizontal bar at top

### 3. Per-Material Greedy Meshing

**Problem**: Original greedy meshing merged faces across different materials, causing UV stretching when a single quad spanned multiple texture atlas regions.

**Solution**: Limit greedy meshing to only merge faces with the **same material type**.

```rust
// Only merge adjacent faces if voxel_type matches
while u + width < CHUNK_SIZE && mask[v][u + width] == Some(voxel_type) {
    width += 1;
}
```

**Impact**:
- Slightly more quads than unconstrained greedy meshing
- Eliminates texture stretching across material boundaries
- Standard industry tradeoff (used by Minecraft and other voxel engines)

## Face-Specific UV Transformations

### The Orientation Problem

Different cube faces use different 3D axis mappings (X→Y, Y→Z, Z→X, etc.), causing textures to appear mirrored or rotated without correction. The goal is consistent texture orientation: **all faces show textures upright when viewed head-on**.

### Orientation Convention: North-Facing Perspective

**North** is defined as the **negative Z direction** (Vec3::NEG_Z). This matches the typical initial camera view in many 3D applications.

**Y-axis faces (top/bottom)** are oriented to be **readable from the north**:
- When looking down at Y+ (top face) from above with camera facing north (-Z), text appears upright
- When looking up at Y- (bottom face) from below with camera facing north (+Z from below), text appears upright

### UV Correction Function

The `correct_uv_orientation()` function applies face-specific transformations:

```rust
fn correct_uv_orientation(
    axis: usize,
    back_face: bool,
    corner_uvs: &[[f32; 2]; 4],
) -> ([f32; 4], [f32; 4])
```

**Corner order**: [0]=bottom-left, [1]=bottom-right, [2]=top-right, [3]=top-left

**Transformations**:
- **Horizontal flip**: swap indices [0,1,2,3] → [1,0,3,2]
- **Vertical flip**: swap indices [0,1,2,3] → [3,2,1,0]
- **90° CCW rotation**: [0,1,2,3] → [1,2,3,0]
- **90° CW rotation**: [0,1,2,3] → [3,0,1,2]

### Face-Specific Transformations

| Face | Axis | Back | Transformation | Reason |
|------|------|------|----------------|---------|
| Z+ (front) | 2 | false | None | Reference face - already correct |
| Z- (back) | 2 | true | Horizontal flip | Viewing from behind |
| X+ (right) | 0 | false | Rotate 90° CCW | Axis mapping rotation |
| X- (left) | 0 | true | Rotate 90° CW + V-flip | Axis mapping + viewing from opposite side |
| Y+ (top) | 1 | false | Vertical flip | Readable from north perspective |
| Y- (bottom) | 1 | true | None | Already correct from north perspective |

**Implementation**:
```rust
let (u, v) = match (axis, back_face) {
    (0, false) => {
        // X+ (right face): rotate 90° CCW
        ([orig_u[1], orig_u[2], orig_u[3], orig_u[0]],
         [orig_v[1], orig_v[2], orig_v[3], orig_v[0]])
    }
    (0, true) => {
        // X- (left face): rotate 90° CW, then flip vertically
        ([orig_u[3], orig_u[0], orig_u[1], orig_u[2]],
         [orig_v[0], orig_v[3], orig_v[2], orig_v[1]])
    }
    (1, false) => {
        // Y+ (top face): flip vertically for north perspective
        (orig_u, [orig_v[3], orig_v[2], orig_v[1], orig_v[0]])
    }
    (1, true) => {
        // Y- (bottom face): no transformation needed
        (orig_u, orig_v)
    }
    (2, false) => {
        // Z+ (front face): no change (reference)
        (orig_u, orig_v)
    }
    (2, true) => {
        // Z- (back face): flip horizontally
        ([orig_u[1], orig_u[0], orig_u[3], orig_u[2]], orig_v)
    }
    _ => (orig_u, orig_v)
};
```

### X-Axis UV Swapping

**Problem**: For X-axis faces, the world-space axes (u_axis=Y, v_axis=Z) don't align with texture space (U=horizontal, V=vertical).

**Solution**: Swap u_size and v_size when calculating texture coordinates for X-axis faces:

```rust
let (texture_u_voxels, texture_v_voxels) = if axis == 0 {
    // X faces: swap because u_axis=Y (vertical) but U is horizontal in texture
    (v_size / voxel_size, u_size / voxel_size)
} else {
    // Y and Z faces: no swap needed
    (u_size / voxel_size, v_size / voxel_size)
};
```

## Greedy Meshing Considerations

### UV Tiling Correctness

**Problem**: Early implementations mapped UVs to [0.0, 1.0] range regardless of quad size, causing texture stretching on non-square merged quads.

**Solution**: Scale UVs proportionally to the number of voxels merged:

```rust
// For a 4×2 voxel quad (4 wide, 2 tall):
let texture_u_voxels = 4.0;  // 4 voxels wide
let texture_v_voxels = 2.0;  // 2 voxels tall

// UVs will range from 0.0 to 4.0 in U, 0.0 to 2.0 in V
// GPU Repeat mode tiles the texture correctly
```

**Result**: A 16×4 voxel wall shows 16×4 texture tiles, not a stretched single tile.

### Winding Order for Y-Axis Faces

Y-axis faces require special winding order handling due to axis orientation:

```rust
// Y-axis faces need inverted winding order
let needs_flip = (axis == 1) ^ back_face;

if needs_flip {
    // Flip winding order
    indices.extend_from_slice(&[
        base_index,
        base_index + 2,
        base_index + 1,
        base_index,
        base_index + 3,
        base_index + 2,
    ]);
} else {
    indices.extend_from_slice(&[
        base_index,
        base_index + 1,
        base_index + 2,
        base_index,
        base_index + 2,
        base_index + 3,
    ]);
}
```

## Testing Approach

### Systematic Visual Verification

The UV mapping system was debugged using a rigorous systematic approach documented in `CLAUDE.md`. Key principles:

**1. Isolated Test Scenes**
- Single simple object (1×1×1 cube)
- Clear directional texture (letter "F")
- Known world-space position

**2. Perpendicular Camera Views**
- Keyboard shortcuts (1-6) to view each face directly
- Documented camera positions for reproducibility
- Console output indicating current view

**3. Directional Test Texture: Letter "F"**

Why "F" works well:
- **Asymmetric**: Only ONE correct upright orientation
- **Obvious vertical bar**: Left side, easy to spot horizontal mirroring
- **Horizontal strokes**: Extend right, easy to spot vertical flipping
- **No rotational symmetry**: Clearly shows 90° rotations

**F Pattern** (8×8 grid):
```
X X X X X X X .
X X . . . . . .
X X . . . . . .
X X X X X . . .
X X . . . . . .
X X . . . . . .
X X . . . . . .
. . . . . . . .
```

**4. Progressive Testing**

Build up complexity incrementally:
1. Single 1×1×1 voxel cube - verify all 6 faces
2. 2×1×1 horizontal bar - verify tiling on Y and Z faces
3. 1×2×1 vertical tower - verify tiling on X and Z faces
4. Larger merged quads - verify greedy meshing doesn't break tiling

### Example Test Cases

**Test 1: Single Cube** (`uv_debug_single_face.rs`)
- 1×1×1 voxel cube at origin
- All 6 faces should show upright "F"
- Camera shortcuts 1-6 for face-by-face inspection

**Test 2: Horizontal Bar** (`uv_debug_2x1_horizontal.rs`)
- 2×1×1 voxel bar (2 voxels side-by-side in X direction)
- X faces: 1 "F" per face
- Y and Z faces: 2 "F"s tiled horizontally

**Test 3: Vertical Tower** (`uv_debug_1x2_tower.rs`)
- 1×2×1 voxel tower (2 voxels stacked in Y direction)
- Y faces: 1 "F" per face
- X and Z faces: 2 "F"s tiled vertically

### Camera Position Reference

For a cube centered at `(0.5, 0.5, 0.5)`:

```rust
let distance = 3.0;
let cube_center = Vec3::new(0.5, 0.5, 0.5);

// X+ (right): Camera looks from +X toward center
Transform::from_xyz(distance + 0.5, 0.5, 0.5)
    .looking_at(cube_center, Vec3::Y);

// X- (left): Camera looks from -X toward center
Transform::from_xyz(-distance + 0.5, 0.5, 0.5)
    .looking_at(cube_center, Vec3::Y);

// Y+ (top): Camera looks down from +Y, up vector = -Z (north)
Transform::from_xyz(0.5, distance + 0.5, 0.5)
    .looking_at(cube_center, Vec3::NEG_Z);

// Y- (bottom): Camera looks up from -Y, up vector = +Z (north from below)
Transform::from_xyz(0.5, -distance + 0.5, 0.5)
    .looking_at(cube_center, Vec3::Z);

// Z+ (front): Camera looks from +Z toward center
Transform::from_xyz(0.5, 0.5, distance + 0.5)
    .looking_at(cube_center, Vec3::Y);

// Z- (back): Camera looks from -Z toward center
Transform::from_xyz(0.5, 0.5, -distance + 0.5)
    .looking_at(cube_center, Vec3::Y);
```

### BRP-Enabled Automated Testing

Use Bevy Remote Protocol for automated screenshot capture:

```bash
# Launch app with BRP enabled
cargo run --example uv_debug_single_face --release

# Send keyboard input to view specific face
mcp__bevy_brp__brp_extras_send_keys(keys=["Digit1"])  # View X+ face

# Capture screenshot
mcp__bevy_brp__brp_extras_screenshot(path="/tmp/face_X+.png")
```

## Critical Learnings

### Issue 1: Camera Framing

**Problem**: Default camera at origin looking at origin resulted in cube being off-center and partially clipped.

**Root Cause**: 1×1×1 voxel cube extends from (0,0,0) to (1,1,1), so center is at (0.5, 0.5, 0.5), NOT origin.

**Solution**: Always calculate and target the actual center of the object:
```rust
let cube_center = Vec3::new(0.5, 0.5, 0.5);
Transform::from_xyz(0.5, 0.5, 3.5).looking_at(cube_center, Vec3::Y)
```

**Lesson**: Proper camera framing is CRITICAL for visual verification. Clipped views can hide orientation issues.

### Issue 2: V-Axis Flip

**Problem**: Textures appeared upside down - letter "F" had long horizontal bar at bottom instead of top.

**Root Cause**: Image storage (row 0 = top) vs UV convention (V=0 = bottom) mismatch.

**Solution**: Flip V coordinates in corner_uvs array.

**Lesson**: Always verify BOTH horizontal AND vertical orientation. Don't assume fixing one axis is sufficient.

### Issue 3: False Confidence from Incomplete Testing

**Problem**: Initial "fix" only addressed horizontal mirroring on X-axis faces, missed vertical flip on ALL faces.

**Root Cause**: Failed to properly analyze screenshots due to poor framing and incomplete visual parsing.

**Solution**: Collaborate with user on visual analysis - share screenshots and compare interpretations.

**Lesson**: Never claim a fix is complete without systematic verification of ALL faces and dimensions.

### AI Vision Limitations with 3D Graphics

AI vision models have specific limitations when analyzing 3D rendered scenes:

1. Cannot reliably parse clipped/partial views of objects
2. May confuse horizontal and vertical orientation issues
3. Can misidentify which parts of asymmetric patterns are "top" vs "bottom"
4. Requires explicit collaboration with human to verify visual correctness

**Best Practice**: After implementing any graphics fix, capture a screenshot and explicitly ask user to verify visual correctness before claiming success.

## Texture Atlas Support (Future)

The current implementation uses GPU Repeat mode with full [0.0, N] UV ranges. For texture atlas support, the following modifications will be needed:

### Atlas Layout

4 materials in horizontal strip:
```
[Air | Grass | Dirt | Stone]
 0.0   0.25    0.5    0.75   1.0
```

Each material occupies 0.25 of atlas width.

### UV Scaling for Atlas

```rust
const ATLAS_REGION_WIDTH: f32 = 0.25;
let atlas_u_start = get_atlas_u_start(voxel_type);

// Scale tiled UVs to fit within atlas region
let u_tiled = (u_coord % 1.0).abs() * ATLAS_REGION_WIDTH + atlas_u_start;
let v_tiled = v_coord % 1.0;  // V still uses full range
```

### Material-Specific Tiling

Atlas-aware UV mapping must respect material boundaries:
- Per-material greedy meshing (already implemented) ensures quads don't span materials
- Each quad's UVs map to a single atlas region
- Tiling occurs within that region using modulo arithmetic

### Migration Path

Current code has atlas support disabled:
```rust
// TODO: For atlas support, scale U by ATLAS_REGION_WIDTH and add atlas_u_start offset
uvs.push([u_corrected[i], v_corrected[i]]);
```

To enable atlas support:
1. Uncomment `get_atlas_u_start()` function
2. Apply atlas scaling to U coordinates after orientation correction
3. Update material system to use atlas texture handle
4. Test with visual validation examples

## Related Issues

**Closed**:
- nova-89e: Fix UV mapping to work with texture atlas without stretching
- nova-k2q: Fix UV tiling issues with atlas using per-material greedy meshing
- nova-9ni: Fix UV mapping stretching in greedy meshing algorithm
- nova-spw: Create texture stretching test examples
- nova-zu1: Generate procedural textures for voxel atlas at runtime
- nova-m4c: Implement texture atlas support in voxel meshing

**In Progress**:
- nova-e2p: Build UV mapping from fundamentals with debuggable test cases
- nova-3a7: Build UV mapping from fundamentals with debuggable test cases (duplicate)
- nova-49e: UV mapping: texture tiles at fixed smallest-voxel size

## References

- Implementation: `/Users/luke/workspace/lexun/nova/crates/voxel/src/meshing.rs`
- Test examples: `/Users/luke/workspace/lexun/nova/crates/voxel/examples/uv_debug_*.rs`
- Visual verification workflow: `/Users/luke/workspace/lexun/nova/CLAUDE.md` (Visual Verification Workflow section)
- Key commits:
  - `3fda45a` - Fix Y-axis UV orientation for north-facing perspective
  - `01928fb` - Fix UV orientation across all cube faces
  - `f705250` - Fix UV tiling for greedy-meshed quads
  - `95d3374` - Add working UV tiling with Repeat mode
