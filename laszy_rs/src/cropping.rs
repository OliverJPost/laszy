use crate::Point;

#[derive(Default)]
pub enum CroppingMethod {
    #[default]
    None,
    BoundingBox{lower_left: (f64, f64), upper_right: (f64, f64)},
}

impl CroppingMethod {
    pub fn is_in_bounds(&self, point: &Point) -> bool {
        match self {
            CroppingMethod::None => true,
            CroppingMethod::BoundingBox{lower_left, upper_right} => {
                point.x >= lower_left.0 && point.x <= upper_right.0 &&
                point.y >= lower_left.1 && point.y <= upper_right.1
            }
        }
    }
}

