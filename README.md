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
    ├── voxel/                   # Voxel system (Enshrouded-style)
    ├── procgen/                 # Procedural generation (No Man's Sky-style)
    ├── networking/              # Multiplayer & server meshing (Star Citizen-style)
    ├── physics/                 # Physics integration
    ├── rendering/               # Rendering optimizations
    ├── engine/                  # Integration layer that combines all components
    └── example/                 # Example/testing application
```

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

# Run the example application
cargo run -p example
```

## Development

This is an experimental project exploring cutting-edge game development
techniques for galaxy-scale voxel worlds. Each engine component can be developed
and tested independently, with the `example` application serving as an
integration testbed.
