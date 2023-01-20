extern crate laszy as laszy_rs;

use laszy_rs::PointCloud as _PointCloud;
use laszy_rs::PointCloudBuilder as _PointCloudBuilder;
use numpy::PyArray;
use pyo3::prelude::*;
use std::ops::DerefMut;

#[pyclass]
struct PointCloud {
    cloud: _PointCloud,
}

#[pymethods]
impl PointCloud {
    #[staticmethod]
    pub fn new() -> Self {
        PointCloud {
            cloud: _PointCloud::new(),
        }
    }

    #[getter]
    pub fn points<'py>(&self, py: Python<'py>) -> PyResult<&'py PyArray<f64, ndarray::Ix2>> {
        let mut xyz = ndarray::Array2::<f64>::zeros((self.cloud.points.len(), 3));
        for (i, point) in self.cloud.points.iter().enumerate() {
            xyz[[i, 0]] = point.x;
            xyz[[i, 1]] = point.y;
            xyz[[i, 2]] = point.z;
        }
        Ok(PyArray::from_owned_array(py, xyz))
    }

    #[getter]
    pub fn ground_points<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<&'py PyArray<bool, ndarray::Ix1>> {
        let mut ground_pts = ndarray::Array1::<bool>::default(self.cloud.points.len());
        for (i, point) in self.cloud.points.iter().enumerate() {
            ground_pts[[i]] = point.classification == las::point::Classification::Ground;
        }
        Ok(PyArray::from_owned_array(py, ground_pts))
    }
}

#[pyclass]
struct PointCloudBuilder {
    builder: _PointCloudBuilder,
}

#[pymethods]
impl PointCloudBuilder {
    /// Default constructor for PointCloudBuilder, based on LAS or LAZ file.
    ///
    /// # Arguments
    ///
    /// * `filepath` - Path to LAS or LAZ file. If file doesn't exist, an error will be raised.
    ///
    /// returns: Result<PointCloudBuilder, PyErr>
    ///
    /// # Examples
    ///
    /// ```
    /// builder = PointCloudBuilder.from_file("test.las")
    /// ```
    #[staticmethod]
    pub fn from_file(filepath: String) -> PyResult<Self> {
        let builder = _PointCloudBuilder::from_file(&filepath);
        let builder = match builder {
            Ok(builder) => builder,
            Err(e) => return Err(Self::parse_error_to_python_exception(e.to_string())),
        };
        Ok(PointCloudBuilder { builder })
    }

    /// Configures the builder to use cropping based on a lower left and upper right corner.
    ///
    /// NOTE: This will not actually crop the file, it just configures the builder to do so when
    /// you run a builder.to_*() method.
    ///
    /// # Arguments
    ///
    /// * `lower_left`: Tuple of (x, y) coordinates for lower left corner of bounding box.
    /// * `upper_right`: Tuple of (x, y) coordinates for upper right corner of bounding box.
    ///
    /// returns: Result<PyRefMut<PointCloudBuilder>, PyErr>
    ///
    /// # Examples
    ///
    /// ```
    /// builder = PointCloudBuilder.from_file("test.las")
    /// builder.with_cropping((12.0, 5.0), (14.0, 9.0))
    /// # If you don't run `to_cloud` or another `to_*` method, nothing will happen.
    /// cloud = builder.to_cloud()
    /// ```
    pub fn with_crop(
        mut slf: PyRefMut<Self>,
        lower_left: (f64, f64),
        upper_right: (f64, f64),
    ) -> PyResult<PyRefMut<Self>> {
        slf.builder
            .with_crop(laszy_rs::CroppingMethod::BoundingBox {
                lower_left,
                upper_right,
            });
        Ok(slf)
    }

    /// Configures the builder to discard a percentage of points randomly.
    ///
    /// NOTE: This will not actually thin the file, it just configures the builder to do so when
    /// you run a builder.to_*() method.
    ///
    /// NOTE: This is not a deterministic process. If you run it twice, you will get a different
    /// set and amount of points.
    ///
    /// # Arguments
    ///
    /// * `keep_percentage`: Float between 0.0 and 1.0, representing the percentage of points to keep.
    ///
    /// returns: Result<PyRefMut<PointCloudBuilder>, PyErr>
    ///
    /// # Examples
    ///
    /// ```
    /// builder = PointCloudBuilder.from_file("test.las")
    /// builder.with_thinning_random(0.2)
    /// # If you don't run `to_cloud` or another `to_*` method, nothing will happen.
    /// cloud = builder.to_cloud()
    /// ```
    pub fn with_thinning_random(
        mut slf: PyRefMut<Self>,
        keep_percentage: f64,
    ) -> PyResult<PyRefMut<Self>> {
        let method = laszy_rs::ThinningMethod::Random {
            percent: keep_percentage,
        };
        slf.builder.with_thinning(method);
        Ok(slf)
    }

    /// Configures the builder to only keep every nth point.
    ///
    /// NOTE: This will not actually thin the file, it just configures the builder to do so when
    /// you run a builder.to_*() method.
    ///
    /// # Arguments
    ///
    /// * `nth`: Int representing the number of points to skip between each point kept. Will always
    /// keep the first point.
    ///
    /// returns: Result<PyRefMut<PointCloudBuilder>, PyErr>
    ///
    /// # Examples
    ///
    /// ```
    /// builder = PointCloudBuilder.from_file("test.las")
    /// builder.with_thinning_every_nth(10)
    /// # If you don't run `to_cloud` or another `to_*` method, nothing will happen.
    /// cloud = builder.to_cloud()
    /// ```
    pub fn with_thinning_every_nth(
        mut slf: PyRefMut<Self>,
        nth: usize,
    ) -> PyResult<PyRefMut<Self>> {
        let method = laszy_rs::ThinningMethod::EveryNth { nth };
        slf.builder.with_thinning(method);
        Ok(slf)
    }

    /// Configures the builder to reclassify points to ground or their original classification based
    /// on the cloth simulation filter (CSF) algorithm.
    ///
    /// NOTE: This will not actually reclassify the file, it just configures the builder to do so
    /// when you run a builder.to_*() method.
    ///
    /// # Arguments
    ///
    /// * `rigidness`: Float between 0.0 and 1.0, representing the rigidity of the cloth. 0.0 will
    /// most likely result in all points being classified as ground, while 1.0 will be the strictest
    /// possible classification.
    /// * `cloth_resolution`: Float representing the resolution of the cloth in meters. It is not
    /// recommended to go below 5 meters. A good default is 5.0 or 10.0 meters.
    /// * `distance_threshold`: Float representing the distance threshold in meters where the
    /// simulation will stop. A good default is 0.1 meters.
    ///
    /// returns: Result<PyRefMut<PointCloudBuilder>, PyErr>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    #[args(
        rigidness = "0.5",
        cloth_resolution = "10.0",
        distance_threshold = "0.1"
    )]
    pub fn with_csf_ground_reclassification(
        mut slf: PyRefMut<Self>,
        rigidness: f64,
        cloth_resolution: f64,
        simulation_threshold: f64,
        classification_threshold: f64,
    ) -> PyResult<PyRefMut<Self>> {
        slf.builder.with_csf_ground_reclassification(
            rigidness,
            cloth_resolution,
            simulation_threshold,
            classification_threshold,
        );
        Ok(slf)
    }

    pub fn to_file(&mut self, filepath: String) -> PyResult<()> {
        let re = self.builder.to_file(&filepath);
        match re {
            Ok(_) => Ok(()),
            Err(e) => Err(Self::parse_error_to_python_exception(e.to_string())),
        }
    }

    pub fn to_cloud(&mut self) -> PyResult<PointCloud> {
        let cloud = self.builder.to_cloud();
        match cloud {
            Ok(cloud) => Ok(PointCloud { cloud }),
            Err(e) => Err(Self::parse_error_to_python_exception(e.to_string())),
        }
    }

    pub fn to_dtm_using_csf(
        &mut self,
        filepath: String,
        rigidness: f64,
        grid_resolution_meters: f64,
        distance_threshold: f64,
    ) -> PyResult<()> {
        let re = self.builder.to_dtm_using_csf(
            &filepath,
            rigidness,
            grid_resolution_meters,
            distance_threshold,
        );
        match re {
            Ok(_) => Ok(()),
            Err(e) => Err(Self::parse_error_to_python_exception(e.to_string())),
        }
    }

    #[staticmethod]
    fn parse_error_to_python_exception(e: String) -> PyErr {
        PyErr::new::<pyo3::exceptions::PyException, _>(e)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn laszy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PointCloudBuilder>()?;
    Ok(())
}
