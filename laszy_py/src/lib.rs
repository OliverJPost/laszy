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
    #[staticmethod]
    pub fn from_file(filepath: String) -> PyResult<Self> {
        let builder = _PointCloudBuilder::from_file(&filepath);
        let builder = match builder {
            Ok(builder) => builder,
            Err(e) => return Err(Self::parse_error_to_python_exception(e.to_string())),
        };
        Ok(PointCloudBuilder { builder })
    }

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

    pub fn with_thinning_random(
        mut slf: PyRefMut<Self>,
        percentage: f64,
    ) -> PyResult<PyRefMut<Self>> {
        let method = laszy_rs::ThinningMethod::Random {
            percent: percentage,
        };
        slf.builder.with_thinning(method);
        Ok(slf)
    }

    pub fn with_thinning_every_nth(
        mut slf: PyRefMut<Self>,
        nth: usize,
    ) -> PyResult<PyRefMut<Self>> {
        let method = laszy_rs::ThinningMethod::EveryNth { nth };
        slf.builder.with_thinning(method);
        Ok(slf)
    }

    pub fn with_csf_ground_reclassification(
        mut slf: PyRefMut<Self>,
        rigidness: f64,
        cloth_resolution: f64,
        distance_threshold: f64,
    ) -> PyResult<PyRefMut<Self>> {
        slf.builder.with_csf_ground_reclassification(
            rigidness,
            cloth_resolution,
            distance_threshold,
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
