# Pwengine

A small graphics engine written in Rust using modern OpenGL.

This project is organized as a Cargo workspace with a ``reusable engine crate`` and a separate game crate for demonstrating engine usage.

## Features

### Rendering

- Modern OpenGL rendering pipeline
- Runtime shader compilation and management
- Custom shader support
- Material abstraction
- Texture loading and binding
- Basic lighting
- Toon shading

### Engine Systems

- Scene management
- Transform system
- Camera system
- Asset management
- Mesh abstraction
- Procedural geometry generation
- Game/engine separation through Cargo workspaces

### Built With

- Rust
- Cargo
- glfw
- glow
- glam
- image

## Project Structure

```text
.
├── core/      # Engine library
└── game/      # Example game/application
```

## Build

```bash
cargo build
```

## Run

```bash
cargo run -p game
```

## Roadmap

### Rendering

- [ ] PBR materials
- [ ] Shadow mapping
- [ ] Instanced rendering
- [ ] Model loading (GLTF/OBJ)
- [ ] Skybox support

### Engine

- [ ] ECS
- [ ] Hierarchical scene graph
- [ ] Asset hot reloading
- [ ] Serialization

### Gameplay

- [ ] Physics integration

## Completed

- [x] Texture support
- [x] Lighting
- [x] Multiple object rendering
- [x] Custom shader support
- [x] Engine/game crate separation
- [x] Cargo workspace architecture
