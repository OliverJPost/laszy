#[derive(Default)]
pub enum ThinningMethod{
    #[default]
    None,
    Random { percent: f64 },
    EveryNth { nth: usize },
    EveryNthRandom { nth: usize },
    Grid2D { cell_amount: usize, max_points_per_cell: usize },
    Grid3D { cell_amount: usize, max_points_per_cell: usize },
}