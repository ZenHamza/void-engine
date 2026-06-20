fn main() {
    println!("Void Engine v{}", void_engine::VERSION);
    println!("A real-time 3D physics engine built with Rust and WebGPU");

    let mut sim = void_engine::physics::FluidSim::new(1024);
    println!("Initialized fluid simulation with {} particles", sim.count());

    for step in 0..100 {
        sim.step(0.016);
        if step % 10 == 0 {
            println!("Step {}: first particle at {:?}", step, sim.particles[0].position);
        }
    }
    println!("Simulation complete.");
}
