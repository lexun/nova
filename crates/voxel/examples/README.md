# Voxel Examples Guide

This directory contains examples demonstrating the voxel rendering system, from basic usage to advanced features.

## Getting Started

**New to voxel rendering? Start here:**

1. **`basic_chunk.rs`** - Create your first voxel structure
   - Demonstrates manual chunk creation and voxel manipulation
   - Creates a pyramid with layered materials (Stone → Dirt → Grass)
   - Perfect introduction to the low-level voxel API

2. **`simple_terrain.rs`** - Generate heightmap terrain (2D)
   - Uses high-level `VoxelTerrain::planar()` API
   - One-line terrain spawning with sensible defaults
   - Recommended for 2D terrain (hills, valleys)

3. **`octree_terrain.rs`** - Generate octree terrain (3D)
   - Uses high-level `VoxelTerrain::cubic()` API
   - Supports unlimited height and 3D features
   - Recommended for caves, overhangs, tall mountains

## Building and Running

```bash
# Build and run any example
cargo run --example <name> --release

# Examples:
cargo run --example basic_chunk --release
cargo run --example simple_terrain --release
cargo run --example uv_debug_single_face --release
```

## Examples by Category

### UV Mapping & Textures

Test and verify texture mapping on voxel meshes.

- **`uv_debug_single_face.rs`** ⭐ Primary UV test
  - Systematic face-by-face inspection of 1×1×1 cube
  - Keyboard shortcuts (1-6) for perpendicular face views
  - BRP-enabled for automated screenshot testing
  - Uses "F" directional texture for clear orientation
  - **Documented in:** `crates/voxel/docs/uv_mapping.md`

- **`uv_debug_1x2_tower.rs`** - Vertical UV tiling test
  - Tests 2 voxels stacked vertically (Y-axis)
  - Verifies greedy meshing doesn't break vertical tiling
  - **Documented in:** `crates/voxel/docs/uv_mapping.md`

- **`uv_debug_2x1_horizontal.rs`** - Horizontal UV tiling test
  - Tests 2 voxels side-by-side (X-axis)
  - Verifies greedy meshing doesn't break horizontal tiling
  - **Documented in:** `crates/voxel/docs/uv_mapping.md`

- **`texture_test.rs`** - Comprehensive multi-shape UV test
  - Tests cube, wall, pillar, and stairs with "F" texture
  - Verifies UV mapping on varied greedy-meshed geometry
  - Good regression test for texture mapping changes

- **`atlas_debug.rs`** - Atlas texture inspection utility
  - Non-interactive: generates atlas and saves to `/tmp/atlas_debug.png`
  - Useful for debugging atlas layout
  - Run once to inspect the generated texture atlas

- **`atlas_example.rs`** - Texture atlas usage demonstration
  - Shows how to apply atlas textures to multi-chunk terrain
  - 5×5 grid of chunks with continuous procedural noise
  - Demonstrates atlas integration with voxel meshes

### LOD (Level of Detail) Systems

Understand and configure terrain Level of Detail.

- **`lod_comparison.rs`** ⭐ Educational LOD comparison
  - Side-by-side static comparison of 3 LOD levels
  - Shows same 32m×32m area at different resolutions
  - Demonstrates chunk count scaling (1 → 4 → 16 chunks)
  - Includes performance metrics and timing
  - **Documented in:** `crates/voxel/docs/lod_system.md`

- **`simple_terrain.rs`** - Basic heightmap LOD (2D)
  - Minimal "hello world" for terrain system
  - Uses `VoxelTerrain::planar(512.0)` API
  - Heightmap strategy (stable, proven)
  - **When to use:** 2D terrain without caves/overhangs
  - **Documented in:** `crates/voxel/docs/lod_system.md`

- **`octree_terrain.rs`** - Advanced octree LOD (3D)
  - Uses `VoxelTerrain::cubic(512.0)` API
  - Octree strategy (experimental, 3D features)
  - Supports unlimited height
  - **When to use:** 3D terrain with caves, tunnels, floating islands
  - **Documented in:** `crates/voxel/docs/lod_system.md`

- **`large_scale_terrain.rs`** - Production-scale settings
  - Demonstrates large worlds (2048m × 2048m)
  - Long view distance (1000m)
  - Uses `LodSettings::production()` configuration
  - Shows `with_lod_settings()` builder pattern
  - **Documented in:** `crates/voxel/docs/lod_system.md`

- **`caves.rs`** - Custom terrain generation with caves
  - Demonstrates `CaveTerrainGenerator` with octree
  - Shows density-based 3D voxel generation
  - Camera positioned underground to showcase caves
  - Uses `VoxelTerrain::cubic().with_generator()` pattern
  - **Documented in:** `crates/voxel/docs/lod_system.md`

- **`visual_test.rs`** - Quick visual testing utility
  - Small 64m world for fast iteration
  - Good lighting and camera positioning
  - Optimized for development and debugging
  - Fast load time makes it ideal for testing changes

### Testing & Debugging

Verify correctness and diagnose issues.

- **`chunk_boundaries.rs`** - Chunk alignment testing
  - Creates 2×2 grid of solid chunks
  - Uses wireframe to detect gaps between chunks
  - Critical diagnostic tool for coordinate system issues
  - Tests mesh generation at chunk boundaries

- **`meshing_validation.rs`** - Greedy meshing regression tests
  - Comprehensive test suite with 8 voxel patterns:
    - Solid cube, hollow cube, sphere
    - Stairs, checkerboard, scattered voxels
    - Horizontal layers, terrain-like
  - Essential regression test for meshing algorithm
  - Helps identify edge cases and meshing bugs

### Advanced Examples

Low-level implementations and internal demonstrations.

See `examples/advanced/` directory:

- **`dynamic_lod.rs`** - Manual LOD implementation
  - **NOTE:** Advanced example showing LOD internals
  - **For typical usage:** See `simple_terrain.rs` or `octree_terrain.rs`
  - Shows low-level `ChunkManager` and distance-based region spawning
  - Includes debug UI overlay (F3 key)
  - Educational value for understanding plugin internals

## Example Learning Path

**Recommended progression:**

1. **Start Simple**
   - `basic_chunk.rs` - Understand voxel fundamentals
   - `simple_terrain.rs` - See the high-level API

2. **Understand LOD**
   - `lod_comparison.rs` - Learn LOD principles
   - `large_scale_terrain.rs` - See production settings

3. **Explore 3D Features**
   - `octree_terrain.rs` - Try 3D octree LOD
   - `caves.rs` - See custom terrain generation

4. **Test Textures** (if working on UV mapping)
   - `uv_debug_single_face.rs` - Test orientation
   - `uv_debug_1x2_tower.rs` and `uv_debug_2x1_horizontal.rs` - Test tiling
   - `texture_test.rs` - Comprehensive verification

## BRP Integration

BRP (Bevy Remote Protocol) is enabled in many examples for automated testing:

**BRP-Enabled Examples:**
- All `uv_debug_*` examples
- `texture_test.rs`
- `atlas_debug.rs`
- And most others

**BRP allows:**
- Automated screenshot capture
- Keyboard input injection
- Window title changes
- Remote control for testing

See the Bevy BRP MCP documentation for usage details.

## Controls

**Standard Controls** (most examples):
- **WASD** - Move camera horizontally
- **Q/E** - Move camera up/down
- **Mouse** - Look around (click to capture, Escape to release)

**Special Controls:**
- **UV debug examples (1-6 keys)** - Jump to perpendicular face views
  - 1 = X+ (right), 2 = X- (left)
  - 3 = Y+ (top), 4 = Y- (bottom)
  - 5 = Z+ (front), 6 = Z- (back)
- **dynamic_lod.rs (F3)** - Toggle debug overlay

## Architecture Notes

### Heightmap vs Octree LOD

**Heightmap (2D):**
- ✅ Stable and proven
- ✅ Best for 2D terrain (hills, valleys)
- ✅ Simple and predictable
- ❌ Height limited by voxel size
- ❌ No caves, overhangs, or 3D features

**Octree (3D):**
- ✅ Unlimited height
- ✅ Supports caves, tunnels, overhangs
- ✅ True 3D terrain features
- ✅ Memory efficient (only creates chunks where terrain exists)
- ⚠️ Experimental (less tested than heightmap)

**When to use:**
- Use **heightmap** (`VoxelTerrain::planar()`) for 2D terrain
- Use **octree** (`VoxelTerrain::cubic()`) for 3D features

See `crates/voxel/docs/lod_system.md` for detailed comparison.

## Documentation References

**Core Documentation:**
- **UV Mapping:** `crates/voxel/docs/uv_mapping.md`
  - UV orientation system
  - Face transformations
  - Systematic testing workflow

- **LOD Systems:** `crates/voxel/docs/lod_system.md`
  - Heightmap vs octree comparison
  - LOD configuration
  - Performance optimization

- **Spherical Terrain:** `crates/voxel/docs/spherical_terrain.md`
  - Planet rendering approaches
  - Challenges and solutions
  - Future work

**Project Documentation:**
- **Main README:** `README.md` (project root)
- **Visual Verification Workflow:** `CLAUDE.md` (systematic 3D graphics testing)

## Current Example Count

**Main Examples:** 14 examples in `crates/voxel/examples/`
**Advanced Examples:** 1 example in `crates/voxel/examples/advanced/`

**Total:** 15 examples (down from 22 after cleanup)

All examples compile and run successfully.
