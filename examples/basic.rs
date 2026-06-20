fn main() {
    println!("Void Engine Basic Example");
    let mut sim = void_engine::physics::FluidSim::new(512);
    for _ in 0..50 {
        sim.step(0.016);
    }
    println!("Simulation ran with {} particles", sim.count());
}
