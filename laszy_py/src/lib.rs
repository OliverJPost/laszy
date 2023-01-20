extern crate laszy as laszy_rs;

use laszy_rs::PointCloudBuilder as _PointCloudBuilder;
use pyo3::prelude::*;
use std::ops::DerefMut;

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
