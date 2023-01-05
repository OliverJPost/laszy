use std::error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use las::{Read, Reader};
use crate::cropping::CroppingMethod;
use crate::LaszyError;
use crate::metadata::Metadata;
use crate::thinning::ThinningMethod;
use crate::cloud::PointCloud;

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
        Ok(PointCloudBuilder{filepaths: vec![filepath.clone()], metadata, crop: CroppingMethod::None, thinning: ThinningMethod::None})
    }


    pub fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn with_crop(&mut self, crop: CroppingMethod)  -> &mut Self {
        self.crop = crop;
        self
    }

    pub fn with_thinning(&mut self, method: ThinningMethod) -> &mut Self {
        self.thinning = method;
        self
    }

    pub fn to_cloud(&self) -> Result<PointCloud, LaszyError> {
        let mut cloud = PointCloud::new();
        for filepath in &self.filepaths {
            let file = File::open(&filepath)?;
            let mut reader = Reader::new(BufReader::new(file))?;
            let header = reader.header();
            let points = reader.points();
            for point in points {
                let point = point?;
                if !self.crop.is_in_bounds(&point) {
                    continue;
                }
                cloud.add_point(point);
            }
        }
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
        let crop = CroppingMethod::BoundingBox{lower_left: (183_551.47,332_414.45), upper_right:(183_564.09,332_424.13)};
        builder.with_crop(crop);
        let cloud = builder.to_cloud().unwrap();
        assert!(cloud.bounds().min.x > 183_551.47, "min x is {}", cloud.bounds().min.x);
        assert!(cloud.bounds().min.y > 332_414.45, "min y is {}", cloud.bounds().min.y);
        assert!(cloud.bounds().max.x < 183_564.09, "max x is {}", cloud.bounds().max.x);
        assert!(cloud.bounds().max.y < 332_424.13, "max y is {}", cloud.bounds().max.y);
    }
}