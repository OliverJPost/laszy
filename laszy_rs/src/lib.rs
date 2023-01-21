/// A crate for reading, processing, and writing LAS files in a memory-efficient way.
///
/// The crate is designed to be used in a pipeline-like fashion, where the user
/// first creates a `PointCloudBuilder` from a LAS file, then applies a series of
/// transformations to it, and finally writes the result to a new LAS file or
/// to a PointCloud instance.
///
/// # Examples
/// ```
/// use laszy::{PointCloudBuilder, ThinningMethod, CroppingMethod};
/// let cloud = PointCloudBuilder::from_file(&"test.las".to_string())
///     .unwrap()
///     .with_crop(CroppingMethod::BoundingBox {
///         lower_left: (183_551.47, 332_414.45),
///         upper_right: (183_564.09, 332_424.13),
///     })
///     .with_thinning(ThinningMethod::EveryNth { nth: 40 })
///     .with_csf_ground_reclassification(0.5, 5.0, 0.1, 1.0)
///     .to_cloud()
///     .unwrap();
/// ```
///
/// # Features
/// - Read LAS/LAZ files
/// - Write LAS/LAZ files
/// - Crop point clouds
/// - Thin point clouds using a variety of methods
/// - Reclassify ground points using the CSF (Cloth Simulation Filter) method
mod builder;
mod cloud;
mod cropping;
mod csf;
mod error;
mod metadata;
#[cfg(test)]
mod tests;
mod thinning;

pub use builder::PointCloudBuilder;
pub use cloud::PointCloud;
pub use cropping::CroppingMethod;
pub use error::LaszyError;
pub use las::Point;
pub use metadata::Metadata;
pub use thinning::ThinningMethod;
