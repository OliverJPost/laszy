use crate::cloud::PointCloud;
use crate::cropping::CroppingMethod;
use crate::csf::surface::ClothSurface;
use crate::metadata::Metadata;
use crate::thinning::ThinningMethod;
use crate::LaszyError;
use las::point::Classification;
use las::{Point, Write};
use las::{Read, Reader};
use std::fs::File;
use std::io::BufReader;

pub struct PointCloudBuilder {
    filepaths: Vec<String>,
    metadata: Metadata,
    crop: CroppingMethod,
    thinning: ThinningMethod,
    csf_filter: Option<(f64, f64, f64)>,
    cloud: Option<PointCloud>,
    writer: Option<las::Writer<File>>,
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
            csf_filter: None,
            cloud: None,
            writer: None,
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

    pub fn with_csf_ground_reclassification(
        &mut self,
        rigidness: f64,
        grid_resolution_meters: f64,
        distance_threshold: f64,
    ) -> &mut Self {
        self.csf_filter = Some((rigidness, grid_resolution_meters, distance_threshold));
        self
    }

    fn perform_csf_simulation(
        &self,
        rigidness: f64,
        cell_resolution: f64,
        distance_threshold: f64,
    ) -> Result<ClothSurface, LaszyError> {
        let (ll, ur) = self.get_crop_corners();
        let top_z = self.metadata.bounds().min.z - 10.0;
        let mut cloth = ClothSurface::initialize(
            ll,
            ur,
            cell_resolution,
            distance_threshold,
            rigidness,
            top_z,
        );

        println!("Creating CSF surface...");
        let pb = indicatif::ProgressBar::new(100);
        let pb_step = (self.metadata.point_count() / 100) as usize;
        let mut count = 0_usize;
        let mut thin_count = 0_usize;
        for filepath in &self.filepaths {
            let file = File::open(&filepath)?;
            let mut reader = Reader::new(BufReader::new(file))?;
            let mut point_iter = reader.points();
            for (i, point) in point_iter.enumerate() {
                if i % pb_step == 0 {
                    pb.inc(1);
                }
                let point = point?;
                if !self.crop.is_in_bounds(&point) {
                    continue;
                }
                if !self.thinning.is_included(thin_count) {
                    thin_count += 1;
                    continue;
                }
                thin_count += 1;

                count += 1;
                cloth.set_max_z_if_closest_to_particle(&point);
            }
        }
        pb.finish();

        if count == 0 {
            return Err(LaszyError::EmptyCloud(
                "The provided cropping and thinning methods resulted in no points being included in the simulation.".to_string()));
        }
        cloth.fix_zero_max_heights();

        println!("Created cloth surface, starting simulation...");
        cloth.simulate(1000);
        Ok(cloth)
    }

    fn get_crop_corners(&self) -> ((f64, f64), (f64, f64)) {
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
        (ll, ur)
    }

    pub fn to_dtm_using_csf(
        &self,
        filepath: &String,
        rigidness: f64,
        grid_resolution_meters: f64,
        distance_threshold: f64,
    ) -> Result<(), LaszyError> {
        let cloth =
            self.perform_csf_simulation(rigidness, grid_resolution_meters, distance_threshold)?;
        cloth.to_asc(filepath);
        Ok(())
    }

    pub fn to_cloud(&mut self) -> Result<PointCloud, LaszyError> {
        self.cloud = Some(PointCloud::new());
        let loaded_points = self.run_building_iterator("Processing points...")?;
        println!(
            "Succesfully loaded {} points into point cloud.",
            loaded_points
        );
        Ok(self.cloud.take().unwrap())
    }

    pub fn to_file(&mut self, filepath: &String) -> Result<(), LaszyError> {
        let file = std::fs::File::create(filepath)?;
        let mut builder = las::Builder::default();
        builder.point_format = self.metadata.point_format().clone();
        let writer = las::Writer::new(file, builder.into_header()?)?;
        self.writer = Some(writer);
        let loaded_points = self.run_building_iterator("Writing points...")?;
        println!("Succesfully wrote {} points to {}", loaded_points, filepath);
        Ok(())
    }

    fn run_building_iterator(&mut self, message: &str) -> Result<usize, LaszyError> {
        let cloth = match self.csf_filter {
            Some((rigidness, grid_resolution_meters, distance_threshold)) => {
                Some(self.perform_csf_simulation(
                    rigidness as f64,
                    grid_resolution_meters,
                    distance_threshold,
                )?)
            }
            None => None,
        };

        let mut pb = indicatif::ProgressBar::new(self.metadata.point_count() as u64);
        println!("{message}");
        let pb_increment = self.metadata.point_count() / 1000;
        let mut count = 0_usize;
        let mut thin_count = 0_usize;
        for filepath in &self.filepaths {
            let file = File::open(&filepath)?;
            let mut reader = Reader::new(BufReader::new(file))?;
            let points = reader.points();
            for (i, point) in points.enumerate() {
                let mut point = point?;
                if i % pb_increment as usize == 0 {
                    pb.inc(pb_increment);
                }
                if !self.crop.is_in_bounds(&point) {
                    continue;
                }
                if !self.thinning.is_included(thin_count) {
                    thin_count += 1;
                    continue;
                }
                thin_count += 1;

                if let Some(ref cloth) = cloth {
                    if cloth.is_ground_point(&point) {
                        point.classification = Classification::Ground;
                    } else {
                        // Only overwrite existing classification if it was classified ground before
                        if point.classification == Classification::Ground {
                            point.classification = Classification::Unclassified;
                        }
                    }
                }

                if self.cloud.is_some() {
                    self.cloud.as_mut().unwrap().add_point(point);
                } else if self.writer.is_some() {
                    self.writer.as_mut().unwrap().write(point)?;
                }

                count += 1;
            }
        }
        pb.finish();
        Ok(count)
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
