# Voxel Examples

Examples are organized into two categories: **demos** for feature showcases and **debug** tools for visual debugging.

## Building and Running

```bash
cargo run -p voxel --example <name> --release
```

## Demos

Feature showcases demonstrating the voxel system capabilities.

| Example | Description | Status |
|---------|-------------|--------|
| `demo_large_terrain` | Production LOD terrain (2048m world, 1000m view distance) | Working |
| `demo_grass_terrain` | Minecraft-style grass on rolling hills | Working |
| `demo_caves` | Cave generation with CaveTerrainGenerator | BROKEN |
| `demo_octree` | 3D octree LOD (unlimited height, caves support) | BROKEN |

## Debug Tools

Visual debugging tools for developing and testing voxel features. These follow a pattern of building complexity: start with the simplest case, verify it works, then test increasingly complex scenarios.

### UV Mapping

Tests texture orientation and tiling on voxel meshes. Uses an "F" directional texture to make orientation issues obvious.

| Example | Description | Tests |
|---------|-------------|-------|
| `debug_uv_single` | Single 1×1×1 voxel | UV orientation on each face |
| `debug_uv_horizontal` | 2×1 horizontal bar | Horizontal texture tiling |
| `debug_uv_vertical` | 1×2 vertical tower | Vertical texture tiling |
| `debug_uv_complex` | Multiple shapes (cube, wall, pillar, stairs) | Greedy meshing + tiling combined |

### Grass Textures

Tests Minecraft-style grass voxels with separate textures for top/bottom/side faces.

| Example | Description | Tests |
|---------|-------------|-------|
| `debug_grass_single` | Single grass voxel | Face texture selection |
| `debug_grass_stack` | Dirt + Grass vertical stack | Material separation, internal face culling |
| `debug_grass_horizontal` | 2×1 horizontal grass | Horizontal texture tiling with grass |

### Other Debug Tools

| Example | Description |
|---------|-------------|
| `debug_lod_comparison` | Side-by-side comparison of 3 LOD levels on same terrain |
| `debug_meshing_patterns` | 8 test patterns for greedy meshing (hollow cube, stairs, etc.) |

## Controls

**Standard Controls** (most examples):
- **WASD** - Move camera horizontally
- **Q/E** - Move camera up/down
- **Mouse** - Look around (click to capture, Escape to release)

**Face Inspection** (debug examples with number keys):
- **1** = X+ (right), **2** = X- (left)
- **3** = Y+ (top), **4** = Y- (bottom)
- **5** = Z+ (front), **6** = Z- (back)

## Documentation

- `crates/voxel/docs/uv_mapping.md` - UV orientation system
- `crates/voxel/docs/lod_system.md` - LOD configuration and strategies
