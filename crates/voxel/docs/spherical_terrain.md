# Spherical Terrain Rendering

This document captures the challenges, approaches, and current state of planet/spherical terrain rendering in Nova's voxel system.

## Table of Contents

1. [Challenges](#challenges)
2. [Proposed Approaches](#proposed-approaches)
3. [Current Implementation](#current-implementation)
4. [Learnings and Constraints](#learnings-and-constraints)
5. [Future Work](#future-work)

---

## Challenges

### 1. Cube-on-Sphere Geometry Problem

**Core Issue**: Cubic voxel chunks cannot tile perfectly on curved surfaces.

- **Geometric impossibility**: Regular cubes positioned on a sphere will always have gaps or overlaps
- **Visible at all densities**: Even at 32×32 chunks per face (6,144 total), gaps remain visible
- **Not a meshing bug**: Flat chunk grids connect seamlessly; the issue is purely geometric curvature

**Evidence**: Testing showed 2×2 flat grid has no gaps, but same chunks on sphere surface show visible separation.

### 2. Cube-Sphere Mapping Distortion

**Cubic mapping approach**: Project 6 cube faces onto sphere surface.

Distortions:
- **Pole distortion**: Chunks near cube corners experience more stretching
- **Face boundaries**: Transition between cube faces can show discontinuities
- **Core convergence**: As depth increases, radial chunks converge toward planet center

**Alternative considered**: Octahedral mapping (8 triangular faces) has better surface distribution but doesn't solve core convergence and adds coordinate transformation complexity.

### 3. Voxel Orientation

**Challenge**: How do voxels align with curved planet surface?

For each voxel position, must calculate:
1. Local position in chunk space
2. Transform to world space using chunk rotation
3. Radial distance from planet center
4. Terrain surface radius at that direction
5. Determine if voxel is "underground" (solid) or "above surface" (air)

**Current solution** (working):
```rust
// Transform to world space
let world_pos = chunk_position + chunk_rotation * local_pos;

// Calculate radial distance
let radial_distance = world_pos.length();

// Get terrain height at surface
let surface_radius = PLANET_RADIUS + terrain_noise(surface_pos);

// Voxel is solid if underground
if radial_distance < surface_radius {
    chunk.set_voxel(x, y, z, solid_voxel);
}
```

### 4. Chunk Positioning and Rotation

**Requirements**:
- Position chunks at correct radial distance from planet center
- Orient chunks so local Y-axis points radially outward
- Adjacent chunks must align for seamless tiling (as much as geometry allows)

**Implemented solution**:
```rust
// Map chunk grid position to sphere direction
let chunk_direction = cube_to_sphere(face, u_center, v_center);

// Position at planet surface (accounting for chunk thickness)
let chunk_position = chunk_direction * (PLANET_RADIUS - chunk_thickness / 2.0);

// Rotate to align with surface normal
let rotation = Quat::from_rotation_arc(Vec3::Y, chunk_direction);
```

### 5. Terrain Height Limitations

**Constraint**: Chunks have fixed thickness (8m for 32³ voxels at 0.25m).

**Problem**: Terrain height variation (±20m) exceeds chunk thickness.

**Implications**:
- Tall mountains may exceed chunk boundaries
- Deep valleys may expose chunk bottoms
- Limited vertical resolution for dramatic terrain

**Potential solutions**:
- Variable chunk thickness based on local terrain variance
- Multiple vertical layers of chunks
- Accept limitation and adjust terrain amplitude

### 6. Performance and Scalability

**Current approach** (planet_chunk_sphere.rs):
- All chunks generated at startup (~10-15s for 6,144 chunks at 32×32 density)
- Blocks window opening until complete
- No LOD support
- Generation time grows quadratically with density

**Limitations**:
- Not viable for large planets
- No streaming or dynamic loading
- Memory usage scales with total planet surface
- Cannot adjust detail based on camera distance

---

## Proposed Approaches

### A. Octree-Based Dynamic Voxel LOD (Recommended)

**Concept**: Adaptive LOD where voxel resolution increases as you approach.

**Key Insight**: Use same terrain generation function at multiple scales.
```rust
// Far:    1 chunk,  1000m world → 31.25m voxels
// Medium: 8 chunks, 500m world  → 15.62m voxels
// Near:   64 chunks, 250m world → 7.81m voxels
```

**Benefits**:
- Perfect visual consistency (same noise at all scales)
- Detail only where needed
- Industry-standard approach for planets
- Solves distant view problem elegantly

**For spherical terrain**:
- 6 root octree nodes (one per cube face)
- Each node subdivides based on distance/visibility
- Maintains cube-sphere mapping but with adaptive density

**Implementation phases**:
1. Parameterize voxel_size in chunk generation ✅ (completed for flat terrain)
2. Multi-scale validation ✅ (validated with lod_comparison example)
3. Octree data structure ✅ (implemented for flat/cubic terrain)
4. Apply to spherical coordinates (in progress)
5. Distance-based split/merge (planned)

**Status**: Foundation complete for flat terrain. Needs adaptation for spherical geometry.

**Related issues**:
- nova-cbs (Dynamic LOD Epic)
- nova-2im (Octree implementation)
- nova-5ge (Apply to spherical surface)

### B. Large Planet + Flat Chunks

**Concept**: If planet is large enough, local chunks can be flat without visible curvature.

**Scale reference**:
- Small planet (500m radius): Horizon ~80m away → curvature very visible
- Medium planet (5km radius): Horizon ~250m away → sweet spot for flat chunks
- Large planet (50km radius): Horizon ~800m away → curvature imperceptible at ground level

**Approach**:
1. Increase PLANET_RADIUS to 5,000m or 50,000m
2. Generate flat chunks in player-local coordinate system
3. Distant view: single sphere mesh
4. Near view: standard flat voxel chunks with LOD

**Benefits**:
- ✅ Simple: flat chunks "just work"
- ✅ Reuses existing flat terrain infrastructure
- ✅ No complex curved geometry handling
- ✅ Standard LOD applies directly

**Drawbacks**:
- ❌ Requires large planets (may not fit all game designs)
- ❌ Distant sphere mesh won't perfectly match terrain
- ❌ Transition from sphere to chunks may show discontinuity

**Status**: Proposed, not implemented.

**Related issues**:
- nova-zf7 (Experiment: Large planet with flat chunks)
- nova-dqz (Documents this approach)

### C. Single Sphere Mesh + Voxel Detail Layer

**Concept**: Separate planet appearance from walkable terrain.

**Architecture**:
1. **Base planet**: Single sphere mesh (icosphere or UV sphere) with procedural displacement
   - 10k-50k vertices
   - Fast to generate
   - Looks good from any distance
2. **Detail layer**: Voxel chunks only within 50m of player for interaction
   - Digging, building, physics
   - Not rendered (hidden by base mesh)
   - Just for gameplay mechanics

**Benefits**:
- ✅ Fast generation
- ✅ Perfect sphere from distance
- ✅ Voxel interaction where needed
- ✅ Simple to implement

**Drawbacks**:
- ❌ Visible pop-in when switching representations
- ❌ Base mesh texture won't match voxel terrain exactly
- ❌ "No Man's Sky problem": what you see from space differs from ground
- ❌ Dual representation increases complexity

**Status**: Proposed, not prioritized.

**Related issues**:
- nova-5gp (Experiment: Single sphere mesh)

### D. Hybrid: Voxel Tapering/Warping

**Concept**: Deform voxel chunks to fit sphere curvature.

**Approach**: Instead of rigid cubic chunks, warp voxel positions to follow sphere surface.
- Maintain regular voxel grid internally
- Apply transformation when generating mesh
- Voxels "taper" as they curve with surface

**Theoretical benefits**:
- Might eliminate gaps
- Maintains voxel-based interaction
- No LOD transition artifacts

**Challenges**:
- Complex transformation math
- Voxel adjacency becomes non-trivial
- Greedy meshing may not work with warped geometry
- Unproven approach, no reference implementations

**Status**: Theoretical only, not explored.

**Mentioned in**: nova-5ge notes

### E. Cubic Sphere Octree (6 Root Nodes)

**Concept**: Combine cubic sphere mapping with octree LOD.

**Structure**:
```
Root level: 6 nodes (one per cube face)
Each face subdivides into octree based on:
- Distance from camera
- View frustum visibility
- Desired LOD level
```

**Benefits**:
- Natural fit for spherical geometry
- Reuses cube-sphere mapping (proven)
- Adaptive LOD per-face
- Can focus detail on visible hemisphere

**Challenges**:
- Face boundary transitions
- Different subdivision depths between faces
- Still subject to cube-on-sphere gaps
- Complex subdivision rules at edges

**Status**: Conceptualized, not implemented.

**Mentioned in**: nova-5ge, nova-cbs

---

## Current Implementation

### Working Code: planet_chunk_sphere.rs

**Location**: `/Users/luke/workspace/lexun/nova/crates/engine/examples/planet_chunk_sphere.rs`

**What works**:
- ✅ Cubic sphere mapping (6 cube faces → sphere)
- ✅ Chunk positioning at correct radial distance
- ✅ Chunk rotation to align with surface
- ✅ Radial distance voxel generation
- ✅ Procedural terrain noise applied spherically
- ✅ 4×4 chunks per face (96 total) renders smoothly

**Current parameters**:
```rust
const PLANET_RADIUS: f32 = 500.0;     // 500m radius (1km diameter)
const VOXEL_SIZE: f32 = 0.25;          // 25cm voxels
const CHUNKS_PER_FACE_EDGE: usize = 4; // 4×4 = 16 per face, 96 total
const CHUNK_SIZE: usize = 32;          // 32³ voxels = 8m chunks
```

**Known issues**:
- ❌ Visible gaps between chunks (geometric limitation)
- ❌ All chunks generated at startup (~5s for 96 chunks)
- ❌ No LOD support
- ❌ Terrain limited to ±20m variation (chunk thickness constraint)
- ❌ Startup blocking

**Key algorithms**:

1. **Cube-to-sphere mapping**:
```rust
fn cube_to_sphere(face: usize, u: f32, v: f32) -> Vec3 {
    let dir = match face {
        0 => Vec3::new(1.0, v, -u),   // +X face
        1 => Vec3::new(-1.0, v, u),   // -X face
        2 => Vec3::new(u, 1.0, -v),   // +Y face
        3 => Vec3::new(u, -1.0, v),   // -Y face
        4 => Vec3::new(u, v, 1.0),    // +Z face
        5 => Vec3::new(-u, v, -1.0),  // -Z face
        _ => Vec3::ZERO,
    };
    dir.normalize()
}
```

2. **Terrain noise** (spherical):
```rust
fn terrain_noise(pos: Vec3) -> f32 {
    // 3D noise applied to surface position
    // Returns height offset in meters (±20m amplitude)
    let scale = 0.05;
    let n1 = (x * 1.2).sin() * (y * 0.9).cos() * (z * 1.5).sin();
    // ... multi-octave noise
    (n1 + n2 * 0.5 + n3 * 0.25) * 20.0
}
```

3. **Voxel generation** (radial):
```rust
// For each voxel in chunk:
let world_pos = chunk_position + chunk_rotation * local_pos;
let radial_distance = world_pos.length();
let surface_radius = PLANET_RADIUS + terrain_noise(surface_pos);

if radial_distance < surface_radius {
    chunk.set_voxel(x, y, z, solid_voxel); // Underground
}
```

### LOD Infrastructure (Planar/Cubic Only)

**Location**: `/Users/luke/workspace/lexun/nova/crates/voxel/src/lod/`

**What exists**:
- ✅ 5-level LOD system (0.25m → 4.0m voxels)
- ✅ Distance-based transitions with hysteresis
- ✅ ChunkManager for region tracking
- ✅ Octree support for cubic terrain
- ✅ High-level API: `VoxelTerrain::planar()` and `VoxelTerrain::cubic()`

**Working examples**:
- `lod_comparison.rs`: Side-by-side LOD levels
- `dynamic_lod.rs`: Real-time distance-based transitions
- `octree_terrain.rs`: 3D octree LOD

**Status**: Fully implemented for flat/cubic terrain. Not yet adapted for spherical.

**API design** (planned for spherical):
```rust
// Vision: should just work
commands.spawn(VoxelTerrain::spherical(500.0));

// With customization
commands.spawn(
    VoxelTerrain::spherical(5000.0)
        .with_lod_settings(LodSettings::planetary())
        .with_generator(MyPlanetGenerator::new())
);
```

---

## Learnings and Constraints

### Geometric Constraints

1. **Cubes cannot tile on spheres**: This is a fundamental geometric truth. No amount of density fixes it.

2. **Flat chunks work if planet is large enough**: Curvature becomes imperceptible beyond certain scale ratios.

3. **Chunk thickness limits terrain amplitude**:
   - 8m chunks → max ±4m terrain variation without overlap
   - Current ±20m terrain needs thicker chunks or multiple layers

### Design Decisions

**Closed issues provide key insights**:

1. **nova-ae4** (Chunk positioning): Proved rotation and positioning math works correctly.

2. **nova-xxm** (Radial voxel generation): Validated radial distance calculation for spherical terrain.

3. **nova-fpp** (Gaps investigation): Definitively proved gaps are geometric, not a meshing bug. Flat chunks connect perfectly.

### Performance Reality

**From experiments**:
- 96 chunks (4×4 per face): ~5s generation, acceptable
- 384 chunks (8×8 per face): ~10s generation, borderline
- 6,144 chunks (32×32 per face): ~15s generation, too slow

**Conclusion**: Upfront generation doesn't scale. LOD and streaming are mandatory for playable planets.

### LOD is Non-Optional

**Key realization** (nova-cbs): For planets visible from space, must support:
- Distant view (orbital): Very low detail, few chunks
- Medium altitude: Moderate detail
- Surface: High detail, many chunks
- Ground: Maximum detail in player vicinity

Without LOD: Either detail is too low from space OR generation is too slow at startup.

### Visual Consistency Critical

**"No Man's Sky problem"** (nova-cbs): Distant terrain that doesn't match close-up is immersion-breaking.

**Solution**: Same procedural generation at all LOD levels. Only resolution changes, not features.

---

## Future Work

### Immediate Next Steps

1. **Adapt octree LOD to spherical geometry** (nova-5ge)
   - Apply working octree system to cube-sphere mapping
   - 6 root nodes, one per cube face
   - Distance-based subdivision per face

2. **Address gaps** - Choose approach:
   - Accept gaps as unavoidable with small planets
   - Move to larger planets + flat chunks (nova-zf7)
   - Explore gap-filling geometry or stitching

3. **Variable chunk thickness** - Allow terrain features that exceed 8m vertical range

### High-Level API Design

**Target user experience**:
```rust
// Simple: just works with defaults
commands.spawn(VoxelTerrain::spherical(5000.0));

// Advanced: full control
commands.spawn(
    VoxelTerrain::spherical(50_000.0)
        .with_lod_strategy(LodStrategy::Octree)
        .with_lod_settings(LodSettings {
            max_distance: 10_000.0,
            levels: 6,
            ..default()
        })
        .with_generator(PlanetGenerator::earth_like())
        .with_region_size(128.0)
);
```

**Blocked by**:
- Spherical octree implementation (nova-5ge)
- LOD streaming system
- Robust gap handling strategy

### Open Questions

1. **Gap mitigation**: How to handle cube-on-sphere gaps?
   - Geometry shader to fill small gaps?
   - Overlapping chunks with z-fighting resolution?
   - Accept visual imperfection?

2. **Multi-layer chunks**: How to handle tall terrain features?
   - Vertical stack of chunks?
   - Variable chunk dimensions?
   - Adaptive thickness based on local terrain variance?

3. **Chunk boundaries at LOD transitions**: How to blend different resolutions?
   - Skirt geometry?
   - Geomorphing?
   - Accept hard transitions with good distance thresholds?

4. **Planet size guideline**: What's the sweet spot?
   - Small (500m): Curvature obvious, gaps visible, requires curved chunks
   - Medium (5km): Good compromise, flat chunks viable locally
   - Large (50km): Essentially flat locally, simple implementation
   - Guidance needed for game designers

5. **Performance budget**: What chunk density is acceptable?
   - Max chunks at highest LOD?
   - Total memory budget?
   - Generation throughput requirements?

### Related Beads Issues

**Keep open**:
- **nova-5ge**: Apply dynamic LOD to spherical planet surface
- **nova-cbs**: Epic: Dynamic LOD system for planet rendering
- **nova-dqz**: Document planet rendering geometry considerations (this document)
- **nova-v6e**: Rethink planet rendering approach
- **nova-5gp**: Experiment: Single sphere mesh with procedural displacement
- **nova-zf7**: Experiment: Large planet with flat chunk generation

**In progress**:
- **nova-2gi**: Build walkable procedural planet prototype
- **nova-8qz**: Procedurally generate and render a small walkable planet
- **nova-1zp**: Design decision: Multi-resolution voxel LOD for planetary-scale worlds

**Completed** (provide foundation):
- **nova-xxm**: Fix voxel generation to use radial distance from planet center ✅
- **nova-ae4**: Fix chunk positioning and orientation on planet surface ✅

### Integration with Existing Systems

**Voxel system strengths to leverage**:
- ✅ Greedy meshing (works great within chunks)
- ✅ Material system (texture atlas)
- ✅ LOD infrastructure (proven for flat terrain)
- ✅ Procedural generation (scale-independent noise)

**Needs adaptation for spherical**:
- Chunk adjacency (across cube face boundaries)
- Face culling (curved surfaces)
- Collision geometry (spherical coordinates)
- Physics (gravity direction varies)

---

## Conclusion

Spherical voxel terrain is achievable but requires careful architecture:

1. **LOD is mandatory** - Upfront generation doesn't scale
2. **Gaps are geometric** - Not a bug, requires design decisions
3. **Scale matters** - Planet size determines flat vs curved approach
4. **Infrastructure exists** - Octree LOD proven for flat terrain
5. **Clear path forward** - Adapt octree to cube-sphere mapping

**Recommended approach**: Start with larger planets (5km+ radius) using flat chunks + octree LOD, then explore spherical octree for smaller planets if needed.

**Vision remains valid**: `commands.spawn(VoxelTerrain::spherical(500.0))` should eventually "just work" with sensible defaults, while exposing full control for advanced users.
