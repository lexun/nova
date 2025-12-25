# Nova

An aspirational collection of [Bevy](https://bevyengine.org/) plugins for building
galaxy-scale, voxel-based worlds with seamless space-to-surface transitions and
multiplayer support.

> **Note**: This is an experimental project by a solo developer, primarily for
> learning and exploring various aspects of game development. The vision below presents
> a set of targets meant to serve as a north star guiding the exploration.
> It is not a roadmap with any expectation of "completion."

## Vision

This project aims to touch on various ideas for a framework that could combine:

- **Voxel system** inspired by Enshrouded's terrain manipulation
- **Procedural generation** for galaxy-scale worlds and systems like No Man's Sky
- **Dynamic server meshing** for multiplayer scalability similar to Star Citizen's goals
- **Seamless transitions** from galactic exploration to planetary surface gameplay

## Architecture

The project is organized as a Rust workspace with crates for each major component:

```
nova/
└── crates/                      # All components and examples
    ├── engine/                  # Integration layer (Bevy app setup, camera, etc.)
    └── voxel/                   # Voxel system with LOD and terrain generation
```

More crates will be added as the project progresses.

### Voxel System

The voxel crate provides:

- **LOD system** with dynamic terrain resolution
- **Greedy meshing** for efficient chunk rendering
- **World-space terrain generation** ensuring consistency across LOD levels
- **Region-based chunk management** for streaming large worlds

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
cargo run -p engine --example basic
```

## Examples

The project includes several examples demonstrating different aspects of the system:

### Voxel System Examples

```bash
# Multi-chunk terrain with chunk boundaries
cargo run -p voxel --example multi_chunk_terrain

# Side-by-side LOD comparison (1, 4, and 16 chunks)
cargo run -p voxel --example lod_comparison

# Dynamic LOD transitions based on camera distance
cargo run -p voxel --example dynamic_lod

# Chunk boundary alignment test
cargo run -p voxel --example chunk_boundaries

# Greedy meshing validation with 8 test patterns
cargo run -p voxel --example meshing_validation
```

### Integration Examples

```bash
# Basic voxel scene with camera controls
cargo run -p engine --example basic

# Planet with spherical chunk distribution (experimental)
cargo run -p engine --example planet_chunk_sphere
```
