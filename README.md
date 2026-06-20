# Void Engine — Real-time 3D Physics Engine

Built with **Rust** and **WebGPU**. Features particle-based fluid simulation and dynamic lighting.

## Features

- **SPH Fluid Simulation** — Smoothed Particle Hydrodynamics with density, pressure, and viscosity
- **WebGPU Compute Pipeline** — High-performance GPU compute for large particle counts
- **WASM Browser Support** — Run the simulation directly in the browser via WebAssembly
- **Dynamic Lighting** — Real-time per-particle lighting with additive blending

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust |
| Graphics API | WebGPU (wgpu) |
| WebAssembly | wasm-bindgen |
| Windowing | winit |

## Quick Start

```bash
# Run the native simulation
cargo run --release

# Run the example
cargo run --release --example basic
```

## WASM Demo

```bash
# Build the WASM target
wasm-pack build --target web

# Start the dev server
cd www
npm install
npm run dev
```

## Project Structure

```
void-engine/
├── Cargo.toml          # Rust package manifest
├── src/
│   ├── main.rs         # Native entry point
│   ├── lib.rs          # Library root
│   ├── physics.rs      # SPH fluid simulation
│   └── particle.rs     # WASM-bindgen particle API
├── examples/
│   └── basic.rs        # Basic simulation example
├── www/                # Web demo (Vite + WASM)
│   ├── index.html
│   ├── package.json
│   └── vite.config.js
└── .github/workflows/  # CI pipeline
```

## License

MIT — see [LICENSE](LICENSE). Copyright 2024 ZenHamza.
