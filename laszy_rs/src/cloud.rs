use crate::pointstructure::PointStructure;
use crate::Point;
use las::Bounds;

pub struct PointCloud {
    points: PointStructure,
    bounds: Bounds,
}

impl PointCloud {
    pub fn new() -> Self {
        PointCloud {
            points: PointStructure::new(),
            bounds: Bounds::default(),
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.bounds.grow(&point);
        self.points.add_point(point);
    }

    pub fn add_points(&mut self, points: Vec<Point>) {
        for point in points {
            self.bounds.grow(&point);
            self.points.add_point(point);
        }
    }

    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }

    pub fn len(&self) -> usize {
        self.points.points.len()
    }
}
