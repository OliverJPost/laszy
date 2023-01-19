pub struct Metadata {
    point_count: u64,
    bounds: las::Bounds,
    point_format: las::point::Format,
    // fIXME transforms: Vector<Transform>,
}

impl Metadata {
    pub fn from_las_header(header: &las::Header) -> Self {
        let point_count = header.number_of_points();
        let bounds = header.bounds();
        let point_format = header.point_format().clone();
        Metadata {
            point_count,
            bounds,
            point_format,
        }
    }

    pub fn point_count(&self) -> u64 {
        self.point_count
    }

    pub fn bounds(&self) -> &las::Bounds {
        &self.bounds
    }

    pub fn center2d(&self) -> (f64, f64) {
        let x = (self.bounds.min.x + self.bounds.max.x) / 2.0;
        let y = (self.bounds.min.y + self.bounds.max.y) / 2.0;
        (x, y)
    }

    pub fn point_format(&self) -> &las::point::Format {
        &self.point_format
    }
}
