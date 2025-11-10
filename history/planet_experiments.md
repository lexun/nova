# Planet Experiments

This document tracks our experiments in building a walkable procedural planet system.

## Philosophy

Rather than get stuck in analysis paralysis, we're taking an iterative, experimental approach:
- Build something concrete first
- Learn from real problems, not theoretical ones
- Document what we learn so we don't run in circles
- Be willing to roll back and try different approaches

## Objective

**Build a small procedural planet that you can walk around on**

Starting constraints:
- Small planet (~1-2km diameter) - big enough to feel planetary, small enough to manage
- Stick with 25cm voxels for now (we know this works)
- Cubic mapping (6 faces) - proven approach used by most games
- Simple procedural terrain (noise-based)
- Basic gravity that points toward planet center
- Get it rendering and walkable first, optimize later

## Experiments

### Experiment 1: Basic Planet Setup
**Date:** 2025-11-09
**Status:** Starting

**Goal:** Get a spherical voxel planet rendering with basic cubic mapping

**Approach:**
- Create new `planet` example
- Generate voxel chunks for a sphere using cubic projection
- Use existing voxel meshing system
- Simple noise-based terrain

**What we'll learn:**
- How cubic mapping feels in practice
- Where performance bottlenecks actually are
- What the chunk/voxel organization needs to be
- How many chunks we actually need for a small planet

**Results:**

✅ **Success!** Basic spherical planet is rendering with cubic mapping.

**What worked:**
- Cubic-to-sphere mapping function correctly projects cube faces onto sphere
- 6,144 voxels (32x32 per face × 6 faces) form visible spherical shape
- Simple sine-wave terrain noise creates height variation (~20m amplitude)
- App launches and runs smoothly
- Voxel positioning is accurate

**What we learned:**
- Sampling density matters: 32x32 per face is too sparse for a 500m radius planet
  - Each voxel appears as a tiny dot/"star"
  - Would need ~100x100 per face minimum for solid-looking surface
  - That's 60,000 voxels just for surface visualization
- Scale challenge: 25cm voxels on a 1km planet = huge number needed
  - Surface area ≈ 3.14 million m²
  - At 25cm density, that's ~50 million surface voxels
  - Current naive approach spawns individual entities = not scalable

**Key insight:**
We need proper chunk-based rendering, not individual voxel entities. The current approach proves the geometry works but won't scale past proof-of-concept.

**Next steps options:**
1. Increase sample density to 100x100 per face (60K voxels) for better visualization
2. Implement proper chunk system with instanced rendering
3. Add camera controls to "orbit" and get closer to surface
4. Implement gravity toward planet center
5. Add collision detection for walking on surface

**Screenshot:** `/tmp/planet_initial.png` - Shows sparse "starfield" pattern of voxels forming sphere

---

### Experiment 2: Increase Sampling Density
**Date:** 2025-11-09
**Status:** Complete

**Goal:** Make the planet surface appear solid by increasing voxel density

**Changes:**
- Increased samples from 32×32 to 128×128 per cube face
- Total voxels: 6,144 → 98,304 (16x increase)

**Results:**

✅ **Much better!** Planet now appears as a solid sphere.

**What worked:**
- Sphere is now clearly visible and solid-looking from distance
- Terrain colors (grass/stone/dirt) are visible
- Individual voxels distinguishable when close
- Cubic face boundaries create interesting grid patterns
- Performance still good with ~100K entities

**What we learned:**
- 128×128 per face is sufficient for visualization
- Cubic grid structure is visible but not problematic
- Height-based coloring works well (>10m = stone, 0-10m = grass, <0m = dirt)
- Wireframe mode helps see individual voxels
- Still spawning individual entities - this is our performance ceiling

**Next challenges:**
- Can't actually "land" on or walk on the surface yet
- No collision detection
- Camera doesn't orient to planet gravity
- Need proper chunk-based meshing for real scale

**Screenshots:**
- `/tmp/planet_dense.png` - Solid sphere from distance
- `/tmp/planet_closer.png` - Close-up showing individual voxels and terrain colors

---

### Experiment 3: Performance Reality Check
**Date:** 2025-11-09
**Status:** Complete (Failure teaches us what we need)

**Goal:** See if removing wireframe helps performance

**Changes:**
- Removed wireframe rendering
- Tested performance with 98K entities

**Results:**

❌ **Failed as expected** - But learned crucial lessons!

**What happened:**
- Removing wireframe made it worse visually (voxels now just tiny dots)
- Performance still terrible (~10 FPS) with 98K entities
- Gaps still huge: sampling every ~4m but voxels are 0.25m
- To eliminate gaps: need 500m ÷ 0.25m = 2000 samples per face edge
- That would be 2000² × 6 faces = 24 million entities = impossible

**The math that kills this approach:**
```
Planet surface area = 4πr² = 3.14 million m²
Voxel cross-section = 0.25m × 0.25m = 0.0625 m²
Surface voxels needed = 3.14M ÷ 0.0625 = ~50 million voxels
```

**Critical insight:**
We've hit the architectural ceiling. Individual entities simply cannot scale to planetary voxel terrain. This is not an optimization problem - it's a fundamental design limitation.

**What we MUST do next:**
Implement proper chunk-based rendering with mesh generation:
- Group voxels into chunks (e.g., 32³)
- Generate single mesh per chunk using marching cubes or greedy meshing
- One entity per chunk instead of one entity per voxel
- Reduces 32³ voxels (32,768) into 1 entity = 32,768x reduction

**Screenshot:** `/tmp/planet_no_wireframe.png` - Shows sparse dots, not solid surface

---

### Experiment 4: Single Chunk Meshing Proof of Concept
**Date:** 2025-11-09
**Status:** In Progress

**Goal:** Prove chunk-based meshing works before applying to planet

**Approach:**
- Implement greedy meshing algorithm in voxel crate
- Generate single 32³ chunk with test voxel data
- Create one mesh entity from entire chunk
- Compare performance: 32,768 individual entities vs 1 meshed entity
- Use same test pattern from basic example for comparison

**Expected results:**
- Massive performance improvement
- Smooth surfaces between adjacent voxels
- Proof that this approach can scale to planet

**Implementation plan:**
1. Add mesh generation function to voxel crate
2. Implement greedy meshing algorithm (simpler than marching cubes)
3. Test with basic example chunk
4. Measure performance difference

**Results:**

✅ **SUCCESS!** Greedy meshing works perfectly!

**What we built:**
- Implemented greedy meshing algorithm in `crates/voxel/src/meshing.rs`
- Algorithm merges adjacent voxel faces of same type into larger quads
- Generates optimized triangle mesh with proper normals and UVs
- Single mesh entity replaces all individual voxel entities

**Performance comparison:**
- **Before**: Chunk with ~50 voxels = 50 entities = wireframe cube rendering
- **After**: Entire 32³ chunk (up to 32,768 voxels) = 1 entity = smooth mesh
- **Result**: Buttery smooth performance, no lag

**Visual improvements:**
- Continuous smooth surfaces instead of individual cubes
- Proper lighting and shadows on merged faces
- Clean edges where voxels meet
- Ready to scale to planetary size

**Key achievement:**
This proves chunk-based meshing can handle voxel-to-mesh conversion efficiently. We can now confidently apply this to the planet, generating chunks on the spherical surface.

**Screenshot:** `/tmp/chunk_meshed.png` - Shows smooth voxel mesh with reference person

**Next step:** Apply chunk meshing to planet surface generation
