# Nova Engine

A modular game engine for building galaxy-scale, voxel-based worlds with
seamless space-to-surface transitions and multiplayer support.

## Vision

This project aims to create a comprehensive game engine that combines:

- **Voxel system** inspired by Enshrouded's terrain manipulation
- **Procedural generation** for galaxy-scale worlds and systems like No Man's Sky
- **Dynamic server meshing** for multiplayer scalability similar to Star Citizen's goals
- **Seamless transitions** from galactic exploration to planetary surface gameplay

## Architecture

The project is organized as a Rust workspace with modular, reusable components:

```
nova/
└── crates/                      # All engine components and examples
    ├── voxel/                   # Voxel system with LOD and terrain generation
    └── engine/                  # Integration layer (Bevy app setup, camera, etc.)
```

### Voxel System

The voxel crate provides:
- **5-level LOD system** with dynamic terrain resolution (0.25m to 4m voxels)
- **Greedy meshing** for efficient chunk rendering
- **World-space terrain generation** ensuring consistency across LOD levels
- **Region-based chunk management** for streaming large worlds
- **Texture atlas system** for material rendering

See `crates/voxel/README.md` for detailed documentation.

### Design Principles

- **Modularity**: Each crate is independent and focused on a specific domain
- **Loose coupling**: Components communicate through well-defined traits and interfaces
- **Reusability**: Engine components can be used across multiple games
- **Performance**: Built with Bevy for high-performance game development

## Getting Started

### Install Nix

```bash
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | \
  sh -s -- install --determinate
```

### Install Direnv

```
nix profile install nixpkgs#direnv
```

Then [add it to your shell](https://direnv.net/docs/hook.html) and restart your
shell session.

### Install Devenv

```
nix profile install nixpkgs#devenv
```

### Run the Project

```bash
# Clone the repository
git clone git@github.com:lexun/nova.git
cd nova

# Allow direnv to load the development environment
direnv allow

# Run the basic example
cargo run --example basic -p engine
```

## Examples

The project includes several examples demonstrating different aspects of the system:

### Voxel System Examples

```bash
# Multi-chunk terrain with chunk boundaries
cargo run --example multi_chunk_terrain -p voxel

# Side-by-side LOD comparison (1, 4, and 16 chunks)
cargo run --example lod_comparison -p voxel

# Dynamic LOD transitions based on camera distance
cargo run --example dynamic_lod -p voxel

# Chunk boundary alignment test
cargo run --example chunk_boundaries -p voxel

# Greedy meshing validation with 8 test patterns
cargo run --example meshing_validation -p voxel
```

### Integration Examples

```bash
# Basic voxel scene with camera controls
cargo run --example basic -p engine

# Planet with spherical chunk distribution (experimental)
cargo run --example planet_chunk_sphere -p engine
```

## Development

This is an experimental project exploring cutting-edge game development
techniques for galaxy-scale voxel worlds. Each engine component can be developed
and tested independently.
