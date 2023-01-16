use std::cell::Cell;
use std::rc::Rc;

const SCALE_FACTORS_SINGULAR: [f64; 15] = [
    0.0, 0.3, 0.51, 0.657, 0.7599, 0.83193, 0.88235, 0.91765, 0.94235, 0.95965, 0.97175, 0.98023,
    0.98616, 0.99031, 0.99322,
];
const SCALE_FACTORS_DOUBLE: [f64; 15] = [
    0.0, 0.3, 0.42, 0.468, 0.4872, 0.4949, 0.498, 0.4992, 0.4997, 0.4999, 0.4999, 0.5, 0.5, 0.5,
    0.5,
];

#[derive(Debug)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub z: Cell<f64>,
    pub max_z: f64,
    pub is_moveable: Cell<bool>,
    pub closest_pt_distance: f64,
}

impl Particle {
    pub fn new(x: f64, y: f64, z: f64, max_z: f64) -> Particle {
        Particle {
            x,
            y,
            z: Cell::new(z),
            max_z,
            is_moveable: Cell::new(true),
            closest_pt_distance: f64::MAX,
        }
    }

    pub fn apply_force(
        &self,
        rigidness: usize,
        neighbours: Vec<&Particle>,
        displacement: f64,
    ) -> f64 {
        let current_z = self.z.get();
        self.apply_internal_force(rigidness, neighbours);
        self.apply_external_force(displacement);
        if self.z.get() > self.max_z {
            self.z.set(self.max_z);
            self.is_moveable.set(false);
        }
        (self.z.get() - current_z).abs()
    }

    fn apply_internal_force(&self, rigidness: usize, neighbours: Vec<&Particle>) {
        for neighbour in neighbours {
            let mut ztransform = self.z_difference(neighbour);
            if neighbour.is_moveable.get() {
                ztransform *= SCALE_FACTORS_DOUBLE[rigidness];
                neighbour.z.set(neighbour.z.get() + ztransform);
            } else {
                ztransform *= SCALE_FACTORS_SINGULAR[rigidness];
            }
            self.z.set(self.z.get() - ztransform);
        }
    }
    fn apply_external_force(&self, displacement: f64) {
        if self.is_moveable.get() {
            self.z.set(self.z.get() + displacement);
        }
    }

    pub fn z_difference(&self, other: &Particle) -> f64 {
        self.z.get() - other.z.get()
    }
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            x: 0.0,
            y: 0.0,
            z: Cell::new(0.0),
            max_z: 0.0,
            is_moveable: Cell::new(true),
            closest_pt_distance: f64::MAX,
        }
    }
}
