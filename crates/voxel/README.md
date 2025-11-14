# Voxel System for Nova Engine

A modular voxel system inspired by Enshrouded's approach to smooth, non-blocky voxel rendering with support for destructible environments and creative building.

## Vision

This voxel system aims to capture the key characteristics that make Enshrouded's voxel system feel natural and non-blocky while providing the foundation for:

- **Destructible environments** with realistic structural behavior
- **Creative building flexibility** without the limitations of traditional block-based systems
- **Smooth terrain manipulation** for organic world shaping
- **Performance at scale** for large open worlds

## Research: Enshrouded's Voxel System

### Technical Architecture

**Proprietary "Holistic" Engine:**

- Custom voxel engine specifically designed by Keen Games for Enshrouded
- Built to handle "immense data and computational requirements" of real-time voxel modification
- Sophisticated systems for storing, retrieving, and updating voxel states
- Coordinate-based architecture using xyz positioning for each voxel

**Key Characteristics:**

- **Small voxel resolution**: Individual voxels are much smaller than traditional block games
- **Non-obvious voxel nature**: Advanced rendering makes it "quite non-obvious that it's voxel based"
- **Rotatable voxels**: Voxels can be oriented in different directions
- **Smooth surface combination**: Multiple small voxels combine to create seamless surfaces
- **Advanced optimization**: Maintains performance during complex construction and terraforming

### What Makes It "Not Feel Like Voxels"

1. **High Resolution**: Smaller individual voxels create more detailed surfaces
2. **Sophisticated Rendering**: Advanced algorithms combine voxels into smooth surfaces during rendering
3. **Flexible Placement**: Mix of piece-by-piece placement and larger "blueprint" chunks
4. **Smart Meshing**: Voxels are rendered together to eliminate the blocky appearance

### Building System Design

**Flexibility Features:**

- Coordinate-based placement system for precision
- Blueprint system with pre-defined shapes and sizes
- Ability to place "bigger blocks" that lay down multiple voxels at once
- Creative freedom without traditional structural integrity constraints

**Technical Challenges:**

- **Precision Building**: Difficulty determining exact voxel boundaries during construction
- **Asymmetry Issues**: Challenges with perfect alignment in complex builds
- **Performance Balance**: Maintaining smooth gameplay during real-time world modification
- **Liquid Simulation**: Complex fluid dynamics in modifiable voxel worlds with networking

### Performance Optimization Strategies

- Advanced algorithms for smooth gameplay during complex constructions
- Sophisticated data structures for efficient voxel state management
- Real-time mesh generation and optimization
- Balancing detailed environments with system accessibility

## Technical Implementation Research

### Smooth Voxel Rendering Techniques

**Dual Contouring:**

- Extension of Surface Nets using derivative information of signed distance fields
- Preserves sharp features (like 90-degree angles) while maintaining smooth surfaces
- Allows both smooth terrain and realistic building structures
- Currently favored in gamedev community for quality results

**Surface Nets:**

- Provides smooth terrain rendering
- Cannot support sharp features, making buildings look unrealistic
- Good for organic terrain but limited for architectural elements

**Marching Cubes:**

- Traditional isosurface extraction algorithm
- Often considered "too blocky" for modern applications
- Generates large amounts of duplicate vertices affecting smooth shading

### Key Technical Decisions

**Voxel Resolution:**

- Higher density (32×32×32 chunks vs Minecraft's 16×16×16) for smoother appearance
- Smaller individual voxel size for more detailed surface representation
- Balance between detail and performance requirements

**Data Structures:**

- Coordinate-based architecture for precise placement
- Chunk-based world organization for streaming and LOD
- Efficient storage and retrieval systems for real-time modification

**Rendering Pipeline:**

- Real-time mesh generation from voxel data
- Advanced meshing algorithms (greedy meshing, surface extraction)
- LOD systems for distant chunks
- Frustum culling for performance optimization

## Design Principles for Nova Engine

### Core Goals

1. **Smooth, Natural Appearance**: Avoid the blocky look of traditional voxel games
2. **Creative Flexibility**: Enable complex building and terrain manipulation
3. **Destructible Environments**: Support realistic destruction and structural behavior
4. **Performance Focus**: Maintain smooth framerates during real-time world modification
5. **Modular Design**: Clean separation between voxel logic and rendering systems

### Technical Considerations

**Structural Integrity:**

- Support both Enshrouded-style "no structural integrity" and realistic physics-based destruction
- Configurable structural integrity modes:
  - **Creative Mode**: No structural constraints, floating structures allowed (like Enshrouded)
  - **Realistic Mode**: Physics-based collapse when support structures are removed
  - **Hybrid Mode**: Configurable rules for different materials or game contexts
- Balance creative freedom with optional realistic environmental behavior

**World Scale:**

- Design for large open worlds with streaming chunk systems
- Efficient memory management for active and inactive chunks
- Network-friendly design for future multiplayer support

**Rendering Quality:**

- Start with basic meshing, iterate toward smooth surface extraction
- Support for multiple material types and textures
- Advanced lighting integration with Bevy's rendering pipeline

## Current Implementation Status

### ✅ Completed Features

**Core Voxel System:**
- Chunk-based world representation (32×32×32 voxel chunks)
- Multiple voxel types (Stone, Dirt, Grass, Sand, Water, Air)
- Greedy meshing algorithm for efficient mesh generation
- Texture atlas system for material rendering
- Continuous terrain generation across chunk boundaries

**Level of Detail (LOD) System:**
- World-space coordinate system for scale-independent terrain
- Dynamic voxel sizing for LOD levels (0.25m to 4m+)
- Chunk count scaling (1 → 4 → 16 chunks for same world area)
- Octree-ready architecture for planet-scale rendering

**Working Examples:**
- `test_terrain_gen.rs` - Multi-chunk terrain with texture atlas (crates/example/src/bin/test_terrain_gen.rs:1)
- `test_multi_scale_terrain.rs` - LOD demonstration with chunk scaling (crates/example/src/bin/test_multi_scale_terrain.rs:1)
- `test_complex_chunks.rs` - Greedy meshing validation
- `test_chunk_gaps.rs` - Chunk boundary testing

### 🚧 In Progress

**Planet Rendering:**
- Octree-based LOD management for spherical worlds
- Seamless space-to-surface transitions
- Chunk subdivision strategies for curved surfaces

### 📋 Planned Features

**Advanced Rendering:**
- Surface Nets or Dual Contouring for smooth organic terrain
- Advanced lighting and material blending
- Ambient occlusion for depth perception

**Interaction System:**
- Real-time voxel placement and removal
- Mesh regeneration on modification
- Structural integrity simulation (configurable modes)

**Performance Optimization:**
- Frustum culling
- Aggressive LOD culling for distant chunks
- Async chunk generation and meshing

## How LOD Works

The voxel system uses **world-space coordinates** as the foundation for scale-independent terrain generation. This ensures terrain consistency across all LOD levels.

### Key Principle: Same World Area, Different Resolution

Instead of showing the same number of chunks at different voxel sizes (which causes height quantization), we show the **same physical world area** with different numbers of chunks:

```rust
// For a 32m × 32m world area:
// Low detail:    1 chunk  with 4.0m voxels (blocky, fast)
// Medium detail: 4 chunks with 1.0m voxels (smooth, moderate)
// High detail:   16 chunks with 0.25m voxels (very smooth, expensive)

let chunk_world_size = CHUNK_SIZE as f32 * voxel_size;
let chunks_needed = (WORLD_AREA_SIZE / chunk_world_size).ceil();
```

### World-Space Terrain Generation

Procedural generation functions operate in **world-space meters**, not voxel coordinates:

```rust
fn terrain_height(world_x: f32, world_z: f32) -> f32 {
    let base = 2.0;
    let hills = (world_x * 0.1).sin() * (world_z * 0.1).cos() * 4.0;
    base + hills  // Returns height in meters
}

// When generating voxels:
let world_x = world_offset_x + (local_x as f32 * voxel_size);
let height_meters = terrain_height(world_x, world_z);
let height_voxels = (height_meters / voxel_size) as usize;
```

This ensures that at world position (16.0, 16.0), the terrain height is **identical** whether using 4m voxels or 0.25m voxels. Only the vertical resolution differs.

### Chunk Count Scaling

Higher LOD requires exponentially more chunks for the same coverage:

- **4× better resolution** = 16× more chunks
- **2× better resolution** = 4× more chunks

This creates a natural performance gradient: only pay the cost of many chunks when the player is close enough to see the detail.

### Example: test_multi_scale_terrain.rs

See `crates/example/src/bin/test_multi_scale_terrain.rs` for a complete working example that demonstrates:
- Three LOD levels side-by-side
- Same world area (32m × 32m) at each level
- Chunk count scaling (1 → 4 → 16)
- Consistent terrain heights across all LOD levels

## Research Areas for Future Investigation

1. **Advanced Meshing Algorithms**: Dual Contouring for smooth terrain while preserving sharp building edges
2. **Structural Physics**: Integration with physics engines for realistic destruction
3. **Networking**: Efficient synchronization of voxel modifications in multiplayer
4. **Procedural Generation**: Noise-based terrain (Perlin/Simplex) for production-quality worlds
5. **Performance Optimization**: Frustum culling, async generation, occlusion culling
6. **Material System**: Material blending, triplanar mapping, advanced texturing

## References and Further Reading

- Enshrouded Developer Interviews and Technical Discussions
- "Dual Contouring of Hermite Data" research papers
- Bevy Engine rendering pipeline documentation
- Game development community discussions on smooth voxel rendering
- Performance optimization techniques for real-time voxel modification

---

_This document serves as the foundation for Nova Engine's voxel system development. It will be updated as we implement features and discover new techniques._
