use crate::pointstructure::PointStructure;
use crate::{LaszyError, Point};
use las::Bounds;
use las::{Color, Read, Reader, Transform, Vector, Write, Writer};
use std::io::BufReader;

pub struct PointCloud {
    pub points: PointStructure,
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

    pub fn to_las(&self, filepath: &String) -> Result<(), LaszyError> {
        println!("Writing to {}", filepath);
        println!("Points: {}", self.points.points.len());
        let mut pb = indicatif::ProgressBar::new(self.points.points.len() as u64);
        let file = std::fs::File::open(&String::from("/Users/ole/Downloads/C_68DN1.LAZ"))?; //fixme
        let mut reader = Reader::new(BufReader::new(file))?;
        let header = reader.header().clone();

        let mut file = std::fs::File::create(filepath).unwrap();
        let mut writer = las::Writer::new(file, header).unwrap();

        let pb_increment = self.points.points.len() / 1000;
        let mut i = 0;
        for point in &self.points.points {
            if i % pb_increment == 0 {
                pb.inc(pb_increment as u64);
            }
            i += 1;
            writer.write(point.clone())?;
        }
        pb.finish_with_message("done");
        Ok(())
    }
}
