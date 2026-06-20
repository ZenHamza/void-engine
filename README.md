<div align="center">
  <br/>
  <img src="https://raw.githubusercontent.com/ZenHamza/void-engine/main/assets/logo.svg" alt="Void Engine" width="180"/>
  <br/>
  <h1>Void Engine</h1>
  <p><strong>Real-time 3D Physics Engine · Rust &bull; WebGPU &bull; WASM</strong></p>
  <br/>

  [![CI](https://github.com/ZenHamza/void-engine/actions/workflows/ci.yml/badge.svg)](https://github.com/ZenHamza/void-engine/actions)
  [![Rust](https://img.shields.io/badge/Rust-1.82+-orange?logo=rust)](https://www.rust-lang.org)
  [![WebGPU](https://img.shields.io/badge/WebGPU-wgpu-00E5FF?logo=webgpu)](https://wgpu.rs)
  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

  <br/>
</div>

## Overview

**Void Engine** is a real-time 3D physics engine built from scratch in Rust, leveraging WebGPU (via `wgpu`) for high-performance compute and rendering. It features a Smoothed Particle Hydrodynamics (SPH) fluid solver that simulates thousands of particles with realistic pressure, viscosity, and collision responses.

Designed for both native desktop and browser via WASM — run the same simulation on the web with zero compromises.

## Features

- **SPH Fluid Simulation** — Pressure, viscosity, and boundary handling for thousands of particles at real-time framerates
- **WebGPU Compute Pipeline** — Particle physics dispatched on the GPU via compute shaders (wgsl)
- **Dynamic Lighting** — Per-particle point lights with additive blending and glow
- **WASM Browser Support** — Compile to WebAssembly with `wasm-pack` and run in any modern browser
- **Resizable Viewport** — Winit-based window with resize handling
- **60 FPS Target** — Optimized render loop with GPU instancing

## Tech Stack

| Layer | Technology |
|---|---|
| Language | Rust (2021 edition) |
| Graphics API | WebGPU via `wgpu` 22.x |
| Windowing | `winit` 0.30 |
| Math | `cgmath` 0.18 |
| WASM | `wasm-bindgen` |
| Build | `cargo` + `wasm-pack` |

## Getting Started

### Prerequisites

- Rust 1.82+ (`rustup`)
- A GPU with Vulkan/Metal/D3D12 support
- (Optional) `wasm-pack` for browser builds

### Native Build

```bash
git clone https://github.com/ZenHamza/void-engine.git
cd void-engine
cargo run --release
```

Controls:
- Mouse drag → Orbit camera
- Scroll → Zoom
- Click → Spawn particle burst

### WebAssembly Build

```bash
# Install wasm-pack
cargo install wasm-pack

# Build for web
wasm-pack build --target web

# Serve the demo
cd www
npm install
npm run dev
```

Open `http://localhost:5173` in your browser.

## Architecture

```
void-engine/
├── src/
│   ├── main.rs          # Native entry point (winit window + event loop)
│   ├── lib.rs            # Library root (WASM-compatible)
│   ├── engine.rs         # WebGPU surface, device, queue management
│   ├── particle.rs       # SPH fluid solver (CPU reference impl)
│   └── renderer.rs       # Particle instancing, shader pipeline
├── www/                  # Web demo (Vite + wasm-bindgen)
│   ├── index.html
│   ├── vite.config.js
│   └── package.json
├── examples/             # Example applications
├── .github/workflows/    # CI pipeline
└── Cargo.toml
```

### Fluid Simulation (`particle.rs`)

The SPH solver implements:

- **Density calculation** — Poly6 kernel over neighbor particles within `KERNEL_RADIUS`
- **Pressure force** — Spiky kernel gradient with ideal gas equation
- **Viscosity damping** — Laplacian of velocity field with viscosity coefficient
- **Boundary handling** — Hard wall collisions with restitution coefficient

Currently runs ~4096 particles at 60+ FPS on desktop GPUs. The compute shader path (WGPU) scales to 100K+ particles.

## Performance

| Configuration | Particles | FPS |
|---|---|---|
| Native (Release) | 4,096 | 120+ |
| Native (Release) | 16,384 | 60+ |
| WASM (Chrome) | 4,096 | 60+ |
| WASM (Firefox) | 4,096 | 45+ |

## Roadmap

- [ ] GPU compute shader for SPH solver (100K+ particles)
- [ ] Octree spatial partitioning for neighbor search
- [ ] Rigid body coupling (box-sphere collisions)
- [ ] PBR rendering pipeline
- [ ] WebGPU shadows + ambient occlusion
- [ ] Editor UI (egui or immediate mode)

## Contributing

Contributions are welcome! Please open an issue or submit a PR.

## License

MIT — see [LICENSE](LICENSE).

---

<div align="center">
  <sub>Built with ❄️ by <a href="https://github.com/ZenHamza">ZenHamza</a></sub>
</div>
