use rand::Rng;

#[derive(Default)]
pub enum ThinningMethod {
    #[default]
    None,
    Random {
        percent: f64,
    },
    EveryNth {
        nth: usize,
    },
    EveryNthRandom {
        nth: usize,
    },
    Grid2D {
        cell_amount: usize,
        max_points_per_cell: usize,
    },
    Grid3D {
        cell_amount: usize,
        max_points_per_cell: usize,
    },
}

impl ThinningMethod {
    pub fn is_included(&self, i: usize) -> bool {
        match self {
            ThinningMethod::None => true,
            ThinningMethod::Random { percent } => {
                let mut rng = rand::thread_rng();
                rng.gen_bool(*percent)
            }
            ThinningMethod::EveryNth { nth } => i % nth == 0,
            ThinningMethod::EveryNthRandom { nth } => {
                let mut rng = rand::thread_rng();
                panic!("Not implemented");
            }
            ThinningMethod::Grid2D {
                cell_amount,
                max_points_per_cell,
            } => {
                panic!("Not implemented");
            }
            ThinningMethod::Grid3D {
                cell_amount,
                max_points_per_cell,
            } => {
                panic!("Not implemented");
            }
        }
    }
}
