# Buffy's Freebuff Workspace

This directory is for Buffy (Codebuff agent) to store working files,
notes, and artifacts while working on the Chumpkin project.

## World Generation Code Location

The world generation code lives in:
- **`crates/pumpkin-world/src/generation/`** - Main generation module
  - `generator/` - World generators (VanillaGenerator, etc.)
  - `noise/` - Noise functions and density functions
  - `surface/` - Surface builders and terrain shaping
  - `structure/` - Structure generation
  - `carver/` - Cave and ravine carving
  - `feature/` - Feature placement
  - `biome.rs` - Biome-related generation
  - `proto_chunk.rs` - Proto-chunk (generation-stage chunk)
- **`crates/pumpkin-world/src/level.rs`** - Level management, chunk fetching
- **`crates/pumpkin-world/src/generation/generator/`** - The `WorldGenerator` trait and `VanillaGenerator` implementation
- **`crates/pumpkin-plugin-api/src/worldgen.rs`** - Plugin API for custom world generators
- **`crates/pumpkin-world/src/world_info/`** - WorldGenSettings, seed management