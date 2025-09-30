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

## Implementation Roadmap

### Phase 1: Foundation

- Basic voxel data structures (`VoxelType`, `Chunk`, `VoxelWorld`)
- Simple greedy meshing for performance baseline
- Integration with existing 3D scene

### Phase 2: Smooth Rendering

- Implement Surface Nets or basic Dual Contouring
- Optimize mesh generation performance
- Add multiple voxel material types

### Phase 3: Interaction System

- Real-time voxel placement and removal
- Mesh regeneration on modification
- Basic structural integrity simulation

### Phase 4: Advanced Features

- Complex destruction physics
- Advanced surface extraction algorithms
- Performance optimization and LOD systems

## Research Areas for Future Investigation

1. **Advanced Meshing Algorithms**: Deeper dive into Dual Contouring implementation
2. **Structural Physics**: Integration with physics engines for realistic destruction
3. **Networking**: Efficient synchronization of voxel modifications in multiplayer
4. **Procedural Generation**: Integration with terrain generation systems
5. **Performance Optimization**: Advanced culling and LOD techniques
6. **Material System**: Support for complex texturing and material blending

## References and Further Reading

- Enshrouded Developer Interviews and Technical Discussions
- "Dual Contouring of Hermite Data" research papers
- Bevy Engine rendering pipeline documentation
- Game development community discussions on smooth voxel rendering
- Performance optimization techniques for real-time voxel modification

---

_This document serves as the foundation for Nova Engine's voxel system development. It will be updated as we implement features and discover new techniques._
