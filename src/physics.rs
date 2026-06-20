use cgmath::{Vector3, vec3};

#[derive(Clone, Copy)]
pub struct FluidParticle {
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub density: f32,
    pub pressure: f32,
}

pub struct FluidSim {
    pub particles: Vec<FluidParticle>,
    pub bounds: f32,
    pub gravity: f32,
}

impl FluidSim {
    pub fn new(count: usize) -> Self {
        let mut rng = rand::thread_rng();
        let particles = (0..count).map(|_| FluidParticle {
            position: vec3(
                rand::Rng::gen_range(&mut rng, -1.0..1.0),
                rand::Rng::gen_range(&mut rng, 0.0..1.5),
                rand::Rng::gen_range(&mut rng, -1.0..1.0),
            ),
            velocity: vec3(0.0, 0.0, 0.0),
            density: 1.0,
            pressure: 0.0,
        }).collect();
        FluidSim { particles, bounds: 2.0, gravity: -9.8 }
    }

    pub fn step(&mut self, dt: f32) {
        for i in 0..self.particles.len() {
            self.particles[i].velocity.y += self.gravity * dt;
            self.particles[i].position += self.particles[i].velocity * dt;

            let p = &self.particles[i];
            for k in 0..3 {
                let mut pos = if k == 0 { p.position.x } else if k == 1 { p.position.y } else { p.position.z };
                let mut vel = if k == 0 { p.velocity.x } else if k == 1 { p.velocity.y } else { p.velocity.z };
                if pos > self.bounds { pos = self.bounds; vel *= -0.3; }
                if pos < -self.bounds { pos = -self.bounds; vel *= -0.3; }
                if k == 1 && pos < 0.0 { pos = 0.0; vel *= -0.3; }
                let p = &mut self.particles[i];
                if k == 0 { p.position.x = pos; p.velocity.x = vel; }
                else if k == 1 { p.position.y = pos; p.velocity.y = vel; }
                else { p.position.z = pos; p.velocity.z = vel; }
            }
        }
    }

    pub fn count(&self) -> usize { self.particles.len() }
}
