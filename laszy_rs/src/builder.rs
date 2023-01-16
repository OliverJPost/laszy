use crate::cloud::PointCloud;
use crate::cropping::CroppingMethod;
use crate::csf::surface::ClothSurface;
use crate::metadata::Metadata;
use crate::thinning::ThinningMethod;
use crate::LaszyError;
use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use las::{Read, Reader};
use std::error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct PointCloudBuilder {
    filepaths: Vec<String>,
    metadata: Metadata,
    crop: CroppingMethod,
    thinning: ThinningMethod,
}

impl PointCloudBuilder {
    /// Initialize a new builder from a Las/Laz file. Will load metadata but no points.
    ///
    /// # Arguments
    ///
    /// * `filepath`: Path to the las/laz file
    ///
    /// returns: Result<PointCloudBuilder, LaszyError>
    ///
    /// # Examples
    ///
    /// ```
    /// use laszy::PointCloudBuilder;
    /// let path = "test.las".to_string();
    /// let builder = PointCloudBuilder::from_file(&path).unwrap();
    /// ```
    pub fn from_file(filepath: &String) -> Result<Self, LaszyError> {
        let file = File::open(&filepath)?;
        let reader = Reader::new(BufReader::new(file))?;
        let header = reader.header();
        let metadata = Metadata::from_las_header(&header);
        Ok(PointCloudBuilder {
            filepaths: vec![filepath.clone()],
            metadata,
            crop: CroppingMethod::None,
            thinning: ThinningMethod::None,
        })
    }

    pub fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn with_crop(&mut self, crop: CroppingMethod) -> &mut Self {
        self.crop = crop;
        self
    }

    pub fn with_thinning(&mut self, method: ThinningMethod) -> &mut Self {
        self.thinning = method;
        self
    }

    fn perform_csf_simulation(
        &self,
        rigidness: usize,
        cell_resolution: f64,
        distance_threshold: f64,
    ) -> Result<ClothSurface, LaszyError> {
        let ll;
        let ur;
        match self.crop {
            CroppingMethod::None => {
                ll = (self.metadata.bounds().min.x, self.metadata.bounds().min.y);
                ur = (self.metadata.bounds().max.x, self.metadata.bounds().max.y);
            }
            CroppingMethod::BoundingBox {
                lower_left,
                upper_right,
            } => {
                ll = (lower_left.0, lower_left.1);
                ur = (upper_right.0, upper_right.1);
            }
        }
        let top_z = self.metadata.bounds().min.z - 10.0;
        let mut cloth = ClothSurface::initialize(
            ll,
            ur,
            cell_resolution,
            distance_threshold,
            rigidness,
            top_z,
        );

        println!("Creating CSF surface");
        let pb = indicatif::ProgressBar::new(100);
        let pb_step = self.metadata.point_count() / 100;
        let mut i = 0;
        for filepath in &self.filepaths {
            let file = File::open(&filepath)?;
            let mut reader = Reader::new(BufReader::new(file))?;
            let mut points = reader.points();
            while let Some(point) = points.next() {
                if i % pb_step == 0 {
                    pb.inc(1);
                }
                i += 1;
                let point = point?;
                if !self.crop.is_in_bounds(&point) {
                    continue;
                }
                //Intentionally don't thin.

                cloth.set_max_z_if_closest_to_particle(&point);
            }
        }
        pb.finish();
        cloth.fix_zero_max_heights();

        println!("Created cloth surface, starting simulation...");

        cloth.simulate(1000);
        Ok(cloth)
    }

    pub fn to_dtm_using_csf(
        &self,
        filepath: &String,
        rigidness: usize,
        grid_resolution_meters: f64,
        distance_threshold: f64,
    ) -> Result<(), LaszyError> {
        let cloth =
            self.perform_csf_simulation(rigidness, grid_resolution_meters, distance_threshold)?;
        cloth.to_asc(filepath);
        Ok(())
    }

    pub fn to_cloud(&self) -> Result<PointCloud, LaszyError> {
        let mut cloud = PointCloud::new();
        let mut pb = indicatif::ProgressBar::new(self.metadata.point_count() as u64);
        let pb_increment = self.metadata.point_count() / 1000;
        let mut loaded_points = 0;
        for filepath in &self.filepaths {
            let file = File::open(&filepath)?;
            let mut reader = Reader::new(BufReader::new(file))?;
            let header = reader.header();
            let points = reader.points();
            for (i, point) in points.enumerate() {
                let point = point?;
                if i % pb_increment as usize == 0 {
                    pb.inc(pb_increment);
                }
                if !self.crop.is_in_bounds(&point) {
                    continue;
                }
                if !self.thinning.is_included(i) {
                    continue;
                }

                cloud.add_point(point);
                loaded_points += 1;
            }
        }
        pb.finish();
        println!("Loaded {} points", loaded_points);
        Ok(cloud)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_builder() -> PointCloudBuilder {
        let path = "test.las".to_string();
        PointCloudBuilder::from_file(&path).unwrap()
    }

    #[test]
    fn test_from_file() {
        let builder = get_test_builder();
        assert_eq!(builder.filepaths.len(), 1);
        assert_eq!(builder.filepaths[0], "test.las".to_string());
    }

    #[test]
    fn test_get_metadata() {
        let builder = PointCloudBuilder::from_file(&"test.las".to_string()).unwrap();
        let metadata = builder.get_metadata();
        assert_eq!(metadata.point_count(), 52_469);
        assert_eq!(metadata.center2d(), (183_557.575, 332_405.407));
    }

    #[test]
    fn test_with_crop() {
        let mut builder = get_test_builder();
        let crop = CroppingMethod::BoundingBox {
            lower_left: (183_551.47, 332_414.45),
            upper_right: (183_564.09, 332_424.13),
        };
        builder.with_crop(crop);
        let cloud = builder.to_cloud().unwrap();
        assert!(
            cloud.bounds().min.x > 183_551.47,
            "min x is {}",
            cloud.bounds().min.x
        );
        assert!(
            cloud.bounds().min.y > 332_414.45,
            "min y is {}",
            cloud.bounds().min.y
        );
        assert!(
            cloud.bounds().max.x < 183_564.09,
            "max x is {}",
            cloud.bounds().max.x
        );
        assert!(
            cloud.bounds().max.y < 332_424.13,
            "max y is {}",
            cloud.bounds().max.y
        );
    }

    #[test]
    fn test_with_thinning() {
        let mut builder = get_test_builder();
        let thinning = ThinningMethod::Random { percent: 0.1 };
        builder.with_thinning(thinning);
        let cloud = builder.to_cloud().unwrap();
        assert!(cloud.len() < 5_400);
    }
}
