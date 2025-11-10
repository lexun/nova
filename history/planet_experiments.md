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
