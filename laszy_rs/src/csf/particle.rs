use std::cell::Cell;

#[derive(Debug)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub z: Cell<f64>,
    pub max_z: f64,
    pub prev_z: f64,
    pub is_moveable: Cell<bool>,
    pub closest_pt_distance: f64,
}

impl Particle {
    pub fn new(x: f64, y: f64, z: f64, max_z: f64) -> Particle {
        Particle {
            x,
            y,
            z: Cell::new(z),
            prev_z: z,
            max_z,
            is_moveable: Cell::new(true),
            closest_pt_distance: f64::MAX,
        }
    }

    pub fn apply_force(&self, rigidness: f64, neighbours: Vec<&Particle>, displacement: f64) {
        self.apply_internal_force(rigidness, neighbours);
        self.apply_external_force(displacement);
        if self.z.get() > self.max_z {
            self.z.set(self.max_z);
            self.is_moveable.set(false);
        }
    }

    fn apply_internal_force(&self, rigidness: f64, neighbours: Vec<&Particle>) {
        for neighbour in neighbours {
            let mut ztransform = self.z_difference(neighbour) / 2.0;
            if neighbour.is_moveable.get() {
                //ztransform *= 0.5; // Halve the force if the neighbour is moveable
                neighbour.z.set(neighbour.z.get() + ztransform * rigidness);
            }
            if self.is_moveable.get() {
                self.z.set(self.z.get() - ztransform * rigidness);
            }
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
            prev_z: 0.0,
            max_z: 0.0,
            is_moveable: Cell::new(true),
            closest_pt_distance: f64::MAX,
        }
    }
}
