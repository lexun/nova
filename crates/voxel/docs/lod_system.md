# LOD System Architecture

This document describes the Level of Detail (LOD) system architecture for Nova's voxel terrain engine, covering both the current heightmap-based system and the experimental octree-based system.

## Table of Contents

1. [Overview](#overview)
2. [LOD Strategies](#lod-strategies)
3. [Heightmap LOD (Current, Stable)](#heightmap-lod-current-stable)
4. [Octree LOD (Experimental, 3D)](#octree-lod-experimental-3d)
5. [Implementation Status](#implementation-status)
6. [Known Issues and Limitations](#known-issues-and-limitations)
7. [Future Enhancements](#future-enhancements)
8. [API Usage](#api-usage)

## Overview

Nova's voxel terrain system supports two distinct LOD strategies, selectable via the `VoxelTerrain` API:

- **Heightmap**: 2D heightmap-based LOD with fixed concentric rings (proven, production-ready)
- **Octree**: 3D octree-based adaptive LOD with view-dependent subdivision (experimental)

Both strategies share the same:
- 5-level LOD system (Lod0-Lod4)
- Exponential voxel size scaling (0.25m → 4.0m)
- Hysteresis-based transition thresholds
- Pluggable terrain generation via `TerrainGenerator` trait

## LOD Strategies

### Strategy Selection

```rust
// Heightmap strategy (2D, stable)
commands.spawn(VoxelTerrain::planar(512.0));

// Octree strategy (3D, experimental)
commands.spawn(VoxelTerrain::cubic(512.0));
```

### LOD Levels

All strategies use the same 5 LOD levels:

| Level | Distance Range | Voxel Size | Chunks per 32m Region | Use Case |
|-------|---------------|------------|---------------------|----------|
| Lod0  | 0-35m         | 0.25m      | 16 (4×4)            | Close detail |
| Lod1  | 35-75m        | 0.5m       | 4 (2×2)             | Near terrain |
| Lod2  | 75-150m       | 1.0m       | 1                   | Medium distance |
| Lod3  | 150-300m      | 2.0m       | 1                   | Far distance |
| Lod4  | >300m         | 4.0m       | 1                   | Horizon |

Distance thresholds are configurable via `LodSettings`:
- `LodSettings::demo()` - 400m view distance (default)
- `LodSettings::low_end()` - 200m view distance (performance)
- `LodSettings::high_end()` - 600m view distance (quality)
- `LodSettings::infinite()` - Unlimited streaming terrain

### Hysteresis

To prevent LOD flickering as the camera moves near thresholds, the system uses a configurable hysteresis buffer (default 5m). A region only changes LOD when distance crosses the threshold by more than the buffer amount.

Example: With 35m threshold and 5m hysteresis:
- Subdivide when distance < 35m
- Merge when distance > 40m
- Stable zone: 35m-40m (no change)

## Heightmap LOD (Current, Stable)

### Architecture

The heightmap system uses a 2D grid of regions for flat or rolling terrain:

```
World
  └─ Regions (x, z) - 2D grid, 32m × 32m
      └─ Chunks at Y=0, extend upward
          └─ Voxels filled from 0 to terrain_height
```

### Key Characteristics

- **Coordinate System**: `RegionCoord { x: i32, z: i32 }` (2D only)
- **Chunk Positioning**: All chunks anchored at Y=0, extend upward
- **LOD Pattern**: Concentric rings around camera (fixed, predictable)
- **Subdivision**: Horizontal only (1 region → 4×4 chunks at Lod0)
- **Terrain Type**: Heightmap (single height value per X/Z coordinate)

### Algorithm

1. **Region Selection**: Check all regions within configurable check radius
2. **Distance Calculation**: Measure distance from camera to region center
3. **LOD Assignment**: Determine target LOD based on distance and hysteresis
4. **Update**: Add/remove/update regions as needed
5. **Chunk Generation**: Spawn chunks for regions requiring them

### Strengths

- **Proven and Stable**: Extensively tested, production-ready
- **Predictable Performance**: Fixed memory usage per region
- **Simple Mental Model**: Easy to reason about and debug
- **Efficient for 2D Terrain**: Optimal for heightmap-based worlds

### Limitations

- **Height Dependent on LOD**: Max height = `CHUNK_SIZE × voxel_size`
  - Lod0 (0.25m voxels): 32 × 0.25m = 8m max height
  - Lod4 (4.0m voxels): 32 × 4.0m = 128m max height
  - **Problem**: Finer detail = lower max height (backwards!)
- **No 3D Features**: Cannot support caves, overhangs, tunnels, or floating islands
- **Memory Waste**: Every chunk extends from Y=0 even if terrain is higher
- **No Vertical LOD**: Detail level is uniform vertically

### Implementation Files

- `/Users/luke/workspace/lexun/nova/crates/voxel/src/lod/manager.rs` - ChunkManager, RegionCoord
- `/Users/luke/workspace/lexun/nova/crates/voxel/src/plugin.rs` - Heightmap spawning/update logic

## Octree LOD (Experimental, 3D)

### Architecture

The octree system uses a 3D hierarchical structure for true volumetric terrain:

```
World
  └─ Octree Nodes (x, y, z) - 3D grid, 64m root nodes
      └─ Subdivide into 8 children (2×2×2) when camera approaches
          └─ Leaf nodes contain chunk meshes
```

### Key Characteristics

- **Coordinate System**: `OctreeCoord { x: i32, y: i32, z: i32 }` (3D)
- **Chunk Positioning**: Chunks positioned anywhere in 3D space
- **LOD Pattern**: View-dependent adaptive (subdivide based on camera distance)
- **Subdivision**: 3D (1 node → 8 children in 2×2×2 pattern)
- **Terrain Type**: Full volumetric (supports caves, tunnels, overhangs)

### Algorithm

1. **Initialization**: Create root nodes around camera (limited vertical range)
2. **Subdivision Check**: For each node, check if camera is within subdivision threshold
3. **Subdivide**: Create 8 child nodes at half size and next LOD level
4. **Merge Check**: For subdivided nodes, check if camera is beyond merge threshold
5. **Merge**: Remove child nodes, recreate parent chunk
6. **Chunk Generation**: Spawn chunks for leaf nodes without entities

### Strengths

- **Unlimited Height**: Add nodes vertically as needed
- **True 3D**: Supports caves, tunnels, overhangs, floating islands
- **Consistent Detail**: Voxel size determines detail in all dimensions
- **Memory Efficient**: Only create chunks where terrain exists
- **View-Dependent**: Better performance (detail only where looking)

### Limitations

- **Experimental Status**: Less tested than heightmap system
- **Complexity**: Harder to reason about and debug
- **Missing Features**: Frustum culling and mesh caching not yet implemented
- **3D Terrain Generator Required**: Needs density-based generation (not just height)

### Data Structure

```rust
pub struct OctreeNode {
    pub coord: OctreeCoord,        // 3D position in octree space
    pub size: f32,                 // Size in meters
    pub lod: LodLevel,             // Detail level
    pub is_subdivided: bool,       // Has children?
    pub children: Vec<OctreeCoord>, // 8 children if subdivided
    pub chunk_entity: Option<Entity>, // Bevy entity if leaf node
}

pub struct OctreeManager {
    nodes: HashMap<OctreeCoord, OctreeNode>,
    root_size: f32,                // Largest node size (e.g., 64m)
    pending_despawn: Vec<Entity>,  // Entities to remove
}
```

### Subdivision Logic

Distance thresholds for subdivision (same as LOD levels):
- Lod4 (4m voxels): subdivide at < 300m
- Lod3 (2m voxels): subdivide at < 150m
- Lod2 (1m voxels): subdivide at < 75m
- Lod1 (0.5m voxels): subdivide at < 35m
- Lod0 (0.25m voxels): maximum detail, no further subdivision

Merge uses hysteresis (+5m) to prevent flickering.

### Implementation Files

- `/Users/luke/workspace/lexun/nova/crates/voxel/src/lod/octree.rs` - OctreeNode, OctreeManager (503 lines)
- `/Users/luke/workspace/lexun/nova/crates/voxel/src/plugin.rs` - Octree spawning logic (spawn_octree_chunks)
- `/Users/luke/workspace/lexun/nova/crates/voxel/src/terrain/generator.rs` - CaveTerrainGenerator for 3D terrain

## Implementation Status

### Completed Features

#### Octree Epic (nova-aij) - **CLOSED**
All planned phases completed:

✅ **Phase 1 (nova-aex)**: Core octree data structure
- OctreeNode struct with 3D positioning
- Subdivision logic (1 → 8 children)
- Distance-based split/merge decisions
- Comprehensive test suite (12 tests)

✅ **Phase 2 (nova-68t)**: 3D chunk management
- OctreeManager for tracking nodes
- Plugin integration with strategy selection
- Backward compatibility with heightmap mode

✅ **Phase 3 (nova-2rh)**: 3D terrain generation
- CaveTerrainGenerator with density-based voxel generation
- Cave carving using 3D noise
- Material layers based on depth

✅ **Phase 4 (nova-806)**: View-dependent LOD with smart visibility
- Frustum culling awareness (basic)
- Visibility toggling instead of immediate despawn
- Mesh entity management

#### High-Level API (nova-egp) - **PARTIALLY COMPLETE**

✅ Completed phases:
- Phase 1 (nova-cie): LOD configuration with presets
- Phase 2 (nova-2ad): TerrainGenerator trait
- Phase 3 (nova-6q8): VoxelTerrain component and plugin

⏸️ Future phase:
- Phase 4 (nova-7hv): Advanced features (async, multi-instance, infinite streaming)

### Working Examples

- `/Users/luke/workspace/lexun/nova/crates/voxel/examples/octree_terrain.rs` - Basic octree terrain
- `/Users/luke/workspace/lexun/nova/crates/voxel/examples/caves.rs` - Cave generation with octree
- `/Users/luke/workspace/lexun/nova/crates/voxel/examples/simple_terrain.rs` - Heightmap terrain
- `/Users/luke/workspace/lexun/nova/crates/voxel/examples/large_scale_terrain.rs` - Large heightmap terrain

### What's Implemented

| Feature | Heightmap | Octree | Notes |
|---------|-----------|--------|-------|
| Basic LOD | ✅ | ✅ | Both fully functional |
| Distance-based subdivision | ✅ | ✅ | Working |
| Hysteresis | ✅ | ✅ | Prevents flickering |
| Chunk spawning | ✅ | ✅ | Fully implemented |
| Mesh generation | ✅ | ✅ | Shared greedy meshing |
| Terrain generation | ✅ | ✅ | Both heightmap and 3D |
| Caves/tunnels | ❌ | ✅ | Octree-only |
| Unlimited height | ❌ | ✅ | Octree-only |
| Frustum culling | ❌ | ⚠️ | Partial (awareness, not full implementation) |
| Mesh caching | ❌ | ❌ | Planned |
| Async generation | ❌ | ❌ | Planned |
| Multi-instance | ❌ | ❌ | Planned |

## Known Issues and Limitations

### Heightmap System Issues

#### OPEN: nova-t5q - Floating Chunks (Chunk Sizing Mismatch)
**Status**: SUPERSEDED by octree implementation
**Symptom**: In `large_scale_terrain` example, highest LOD chunks appear small with gaps
**Root Cause**: Chunk mesh size doesn't match spawn spacing
```rust
// plugin.rs:255, 272
chunk_world_size = region_size / chunks_per_edge  // Spacing
mesh_size = CHUNK_SIZE × voxel_size               // Actual size
```
**Example of mismatch**:
- Region: 32m × 32m, LOD 0: 4×4 chunks, voxel_size = 0.125m
- chunk_world_size = 32 / 4 = 8m spacing
- mesh_size = 32 × 0.125 = 4m
- **Result**: 4m meshes spaced 8m apart = gaps!

**Workaround**: Use octree system or avoid production settings with heightmap
**Long-term Fix**: The octree implementation naturally avoids this issue through proper 3D subdivision

#### OPEN: nova-6gu - Terrain Height Clipping
**Status**: SUPERSEDED by octree implementation
**Symptom**: Terrain gets cut off at certain height, peaks appear flattened
**Root Cause**: Chunks fixed at 32³ voxels, height limit depends on voxel size
```rust
// generator.rs:108
let max_y = height_voxels.min(CHUNK_SIZE - 1); // Clamps to 32 voxels
```
**Impact**:
- Lod0 (0.25m): 32 × 0.25m = 8m max height
- Lod0 (0.125m): 32 × 0.125m = 4m max height
- Terrain can be ~14m tall (base 2m + hills 8m + variation 4m)

**Workaround**: Use octree system for unlimited height
**Long-term Fix**: Octree allows unlimited vertical chunks

### Octree System Issues

#### Missing: Frustum Culling (Planned)
**Status**: Partial implementation, needs completion
**Current**: System is "frustum aware" but doesn't fully implement culling
**Needed**:
- Proper frustum intersection tests
- Don't subdivide nodes outside view frustum
- Hide nodes behind camera instead of despawning

#### Missing: Mesh Caching (Planned)
**Status**: Not implemented
**Impact**: Regenerates meshes when revisiting areas
**Needed**:
- Cache generated meshes by octree coordinate
- Reuse cached meshes when returning to previously visited areas
- Evict cache based on memory pressure

#### Missing: LOD Transition Smoothness
**Status**: No geometric continuity at boundaries
**Symptom**: T-junctions between different LOD levels
```
High-res edge:  ●---●---●---●
Low-res edge:   ●-----------●
                     ^ Gap! (T-junction)
```
**Potential Solutions**:
- Skirt geometry (simple, works for heightmaps)
- Stitching meshes (complex but proper)
- Constrained triangulation (very complex)
- Accept small gaps (may be imperceptible at distance)

**Current Approach**: Accept small gaps - not yet validated if visible

### Both Systems

#### Height Quantization at LOD Transitions (P2 - Lower Priority)
**Symptom**: Terrain "pops" up/down slightly during LOD transitions
**Root Cause**: Different voxel sizes round height differently
```rust
// generator.rs:104
let height_voxels = (height_meters / voxel_size) as usize; // Truncates
```
**Example**: Terrain height of 5.3m
- At 0.25m voxels: 5.3 / 0.25 = 21.2 → 21 voxels → 5.25m rendered
- At 0.5m voxels: 5.3 / 0.5 = 10.6 → 10 voxels → 5.0m rendered
- **Difference**: 0.25m visible pop

**Potential Fix**: Use rounding instead of truncation, or align to coarsest voxel grid

## Future Enhancements

### High Priority (Would Improve Octree)

#### 1. Complete Frustum Culling (nova-806 partial)
**Benefit**: Significant performance improvement
**Implementation**:
- Add frustum intersection test to OctreeManager
- Modify update() to consider frustum when subdividing
- Use Visibility component instead of despawning for out-of-frustum nodes

**Estimated Effort**: ~100 LOC, 2-4 hours

#### 2. Mesh Caching
**Benefit**: Eliminate mesh regeneration when revisiting areas
**Implementation**:
- Add HashMap<OctreeCoord, Handle<Mesh>> to OctreeManager
- Check cache before generating new mesh
- Evict based on LRU or memory pressure

**Estimated Effort**: ~150 LOC, 3-5 hours

#### 3. Fix LOD Boundary T-Junctions
**Benefit**: Seamless visual transitions
**Implementation**: Start with skirt geometry approach
- Add vertical "skirt" faces at chunk edges
- Match to neighbor LOD level

**Estimated Effort**: ~200 LOC, 6-8 hours (complex)

### Medium Priority (API Improvements)

#### 4. Multiple Terrain Instances (nova-7hv)
**Benefit**: Support multiple biomes or separated terrain
**Implementation**:
- Per-terrain ChunkManager/OctreeManager tracking
- Separate materials per instance

**Estimated Effort**: ~100 LOC, 2-3 hours

#### 5. Infinite Terrain Streaming (nova-7hv)
**Benefit**: Minecraft-like endless worlds
**Implementation**:
- Remove world bounds constraint
- Stream regions/nodes around camera
- Ring buffer for loaded areas

**Estimated Effort**: ~150 LOC, 4-6 hours

#### 6. Async Chunk Generation (nova-7hv)
**Benefit**: Smooth framerate during chunk loading
**Implementation**:
- Bevy AsyncComputeTaskPool integration
- Placeholder meshes during generation
- Queue management

**Estimated Effort**: ~100 LOC, 3-5 hours

### Low Priority (Nice to Have)

#### 7. Spherical Terrain (nova-5ge)
**Status**: Basic cubic sphere mapping exists, needs octree integration
**Vision**: `VoxelTerrain::spherical(radius)` for planets
**Challenges**:
- Voxel tapering/warping to fit curvature
- Cubic sphere octree (6 root nodes, one per face)
- Chunk boundary alignment

**Estimated Effort**: ~300 LOC, 8-12 hours (complex)

#### 8. Custom LOD Transition Hooks
**Benefit**: Callbacks for custom effects (fade, animation)
**Implementation**:
- Events before/after LOD changes
- User-configurable transition behavior

**Estimated Effort**: ~50 LOC, 1-2 hours

### Research Needed

#### 9. Octree Performance at Scale
**Question**: How does octree perform with thousands of active nodes?
**Validation**:
- Benchmark with large view distances
- Profile update() and spawning overhead
- Compare to heightmap system

#### 10. Optimal Root Node Size
**Current**: 64m for octree vs 32m for heightmap
**Question**: What's the sweet spot for root node size?
**Factors**: Memory, subdivision depth, update frequency

## API Usage

### Basic Terrain

```rust
use bevy::prelude::*;
use voxel::{VoxelTerrain, VoxelTerrainPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VoxelTerrainPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Heightmap terrain (2D, stable)
    commands.spawn(VoxelTerrain::planar(512.0));

    // OR

    // Octree terrain (3D, experimental)
    commands.spawn(VoxelTerrain::cubic(512.0));
}
```

### Custom Settings

```rust
use voxel::{VoxelTerrain, LodSettings, DefaultTerrainGenerator};

// Customized heightmap terrain
commands.spawn(
    VoxelTerrain::planar(1024.0)
        .with_lod_settings(LodSettings::high_end())
        .with_region_size(64.0)
        .with_generator(
            DefaultTerrainGenerator::new()
                .with_max_world_size(1024.0)
        )
);
```

### Cave Terrain

```rust
use voxel::{VoxelTerrain, CaveTerrainGenerator};

// 3D terrain with caves
commands.spawn(
    VoxelTerrain::cubic(512.0)
        .with_generator(
            CaveTerrainGenerator::new()
                .with_max_world_size(512.0)
                .with_cave_threshold(0.25)    // More caves
                .with_cave_frequency(0.06)    // Larger caves
        )
);
```

### Infinite Streaming Terrain

```rust
use voxel::{VoxelTerrain, LodSettings};

// Infinite terrain (no bounds)
commands.spawn(
    VoxelTerrain::infinite()
        .with_lod_settings(LodSettings::infinite())
);
```

### Custom Terrain Generator

```rust
use voxel::{TerrainGenerator, Chunk, VoxelType, Voxel, CHUNK_SIZE};
use bevy::math::Vec3;

struct FlatTerrainGenerator {
    height: f32,
}

impl TerrainGenerator for FlatTerrainGenerator {
    fn generate_chunk(&self, world_offset: Vec3, voxel_size: f32) -> Chunk {
        let mut chunk = Chunk::new();

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_y = world_offset.y;

                if world_y < self.height {
                    // Fill below height with grass
                    for y in 0..CHUNK_SIZE {
                        if world_offset.y + (y as f32 * voxel_size) < self.height {
                            chunk.set_voxel(
                                x, y, z,
                                Voxel::Solid(VoxelType::Grass)
                            );
                        }
                    }
                }
            }
        }

        chunk
    }
}

// Use custom generator
commands.spawn(
    VoxelTerrain::planar(256.0)
        .with_generator(FlatTerrainGenerator { height: 5.0 })
);
```

## Recommendations

### For Production Use

**Use Heightmap System** if:
- You need proven stability
- Terrain is primarily 2D (hills, valleys)
- Height limits are acceptable (< 32m at finest detail)
- No caves or overhangs needed

**Use Octree System** if:
- You need 3D features (caves, tunnels, overhangs)
- Unlimited terrain height required
- Willing to accept experimental status
- Can contribute to testing and bug fixes

### Migration Path

If starting with heightmap and later need 3D:
1. Develop with `VoxelTerrain::planar()` initially
2. Switch to `VoxelTerrain::cubic()` when needed
3. Update terrain generator to 3D (e.g., CaveTerrainGenerator)
4. No other code changes required (same API)

### Performance Tuning

**For better performance**:
- Use `LodSettings::low_end()` to reduce view distance
- Larger region/root node size reduces subdivision overhead
- Profile with many active chunks to find bottlenecks

**For better quality**:
- Use `LodSettings::high_end()` for extended view distance
- Smaller voxel sizes at Lod0 (requires code change)
- Implement mesh caching to reduce regeneration

## References

### Code Files

**LOD System**:
- `/Users/luke/workspace/lexun/nova/crates/voxel/src/lod/mod.rs` - Module overview
- `/Users/luke/workspace/lexun/nova/crates/voxel/src/lod/level.rs` - LodLevel enum and distance thresholds
- `/Users/luke/workspace/lexun/nova/crates/voxel/src/lod/config.rs` - LodSettings configuration
- `/Users/luke/workspace/lexun/nova/crates/voxel/src/lod/manager.rs` - ChunkManager, RegionCoord (heightmap)
- `/Users/luke/workspace/lexun/nova/crates/voxel/src/lod/octree.rs` - OctreeNode, OctreeManager (octree)

**Terrain Generation**:
- `/Users/luke/workspace/lexun/nova/crates/voxel/src/terrain/mod.rs` - Module overview
- `/Users/luke/workspace/lexun/nova/crates/voxel/src/terrain/generator.rs` - TerrainGenerator trait, implementations

**Plugin**:
- `/Users/luke/workspace/lexun/nova/crates/voxel/src/plugin.rs` - VoxelTerrain, VoxelTerrainPlugin

**Examples**:
- `/Users/luke/workspace/lexun/nova/crates/voxel/examples/simple_terrain.rs` - Basic heightmap
- `/Users/luke/workspace/lexun/nova/crates/voxel/examples/large_scale_terrain.rs` - Large heightmap
- `/Users/luke/workspace/lexun/nova/crates/voxel/examples/octree_terrain.rs` - Basic octree
- `/Users/luke/workspace/lexun/nova/crates/voxel/examples/caves.rs` - Cave generation

### Issue Tracking

**Closed Epics**:
- nova-aij: 3D octree-based LOD system (4 phases completed)
- All phases: nova-aex, nova-68t, nova-2rh, nova-806

**Open Epics**:
- nova-egp: Flexible LOD terrain generation system (Phase 4 pending)

**Open Issues** (superseded by octree):
- nova-t5q: Floating chunks in heightmap system
- nova-6gu: Terrain height clipping in heightmap system

**Future Work**:
- nova-2im: Implement octree as alternative LodStrategy
- nova-5ge: Apply dynamic LOD to spherical planet surface
- nova-7hv: Advanced features (async, multi-instance, infinite streaming)

### Architectural Decisions

**Why Two Systems?**
- Backward compatibility: Don't break existing heightmap terrain
- Gradual migration: Let users choose when to adopt 3D
- Comparison: Validate octree performance against proven baseline
- Risk mitigation: Keep stable option while experimenting

**Why Octree for 3D?**
- Industry standard for voxel games (Minecraft, etc.)
- Natural fit for adaptive detail
- Unlimited height via vertical nodes
- Memory efficient (only create chunks that exist)

**Why Keep Heightmap?**
- Simpler mental model
- Proven performance characteristics
- Optimal for 2D terrain use cases
- Lower complexity for debugging

### Historical Context

The octree system was developed in response to critical bugs discovered in the heightmap system:
1. Terrain height clipping at high LOD levels
2. Floating chunks with gaps between them
3. Height quantization between LOD transitions

These bugs revealed fundamental architectural limitations of 2D heightmaps, particularly the inverse relationship between voxel detail and maximum terrain height. The user's question "What if we had a world like Minecraft with deep tunnels?" crystalized the need for true 3D support.

---

**Document Version**: 1.0
**Last Updated**: 2025-11-29
**Status**: Octree experimental, heightmap stable
