use std::error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use las::{Read, Reader};
use super::LaszyError;
use super::metadata::Metadata;

pub struct PointCloudBuilder {
    filepaths: Vec<String>,
    metadata: Metadata
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
        Ok(PointCloudBuilder{filepaths: vec![filepath.clone()], metadata})
    }

    pub fn get_metadata(&self) -> &Metadata {
        &self.metadata
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
}