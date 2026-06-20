use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ParticleSystem {
    positions: Vec<f32>,
    velocities: Vec<f32>,
    count: usize,
}

#[wasm_bindgen]
impl ParticleSystem {
    #[wasm_bindgen(constructor)]
    pub fn new(count: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut positions = Vec::with_capacity(count * 3);
        let mut velocities = Vec::with_capacity(count * 3);
        for _ in 0..count {
            positions.push(rand::Rng::gen_range(&mut rng, -1.0..1.0));
            positions.push(rand::Rng::gen_range(&mut rng, 0.0..1.5));
            positions.push(rand::Rng::gen_range(&mut rng, -1.0..1.0));
            velocities.push(0.0); velocities.push(0.0); velocities.push(0.0);
        }
        ParticleSystem { positions, velocities, count }
    }

    pub fn update(&mut self, dt: f32) {
        let gravity = -9.8 * dt;
        for i in 0..self.count {
            let i3 = i * 3;
            self.velocities[i3 + 1] += gravity;
            self.positions[i3] += self.velocities[i3] * dt;
            self.positions[i3 + 1] += self.velocities[i3 + 1] * dt;
            self.positions[i3 + 2] += self.velocities[i3 + 2] * dt;

            for k in 0..3 {
                let idx = i3 + k;
                if self.positions[idx] > 2.0 { self.positions[idx] = 2.0; self.velocities[idx] *= -0.3; }
                if self.positions[idx] < -2.0 { self.positions[idx] = -2.0; self.velocities[idx] *= -0.3; }
            }
            if self.positions[i3 + 1] < 0.0 { self.positions[i3 + 1] = 0.0; self.velocities[i3 + 1] *= -0.3; }
        }
    }

    pub fn get_positions(&self) -> Vec<f32> { self.positions.clone() }
    pub fn get_count(&self) -> usize { self.count }
}
