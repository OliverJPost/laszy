use derive_more::{Index, IntoIterator};

#[derive(Default, IntoIterator, Index)]
pub struct PointStructure {
    #[into_iterator]
    pub points: Vec<las::Point>,
}

impl PointStructure {
    pub fn new() -> Self {
        PointStructure::default()
    }

    pub fn add_point(&mut self, point: las::Point) {
        self.points.push(point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_iterator() {
        let mut ps = PointStructure::new();
        let p1 = las::Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            ..Default::default()
        };
        ps.add_point(p1.clone());
        let p2 = las::Point {
            x: 5.0,
            y: 2.2,
            z: 1.5,
            ..Default::default()
        };
        ps.add_point(p2.clone());
        let mut point_iterator = ps.into_iter();
        assert_eq!(point_iterator.next(), Some(p1));
        assert_eq!(point_iterator.next(), Some(p2));
        assert_eq!(point_iterator.next(), None);
    }
}
