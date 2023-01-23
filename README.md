Lazy processing of las/laz files in Rust or Python.
Pronounce as "lazy".

Created for assignment 4 of the [GEO1015](https://3d.bk.tudelft.nl/courses/geo1015/) course.


---

### Installing `laszy`:
##### As Python module:
1. Install latest [Rust](https://www.rust-lang.org/).
2. Install latest [Maturin](https://github.com/PyO3/maturin).
3. In your terminal, activate your Python environment (i.e. venv or conda).
4. `cd` to `laszy_py` directory and run `maturin develop --release`.
5. You can now import `laszy` in your Python code, like this: `from laszy import PointCloudBuilder`.

##### As Rust library crate:
1. Add the local path to `laszy_rs` to your `Cargo.toml` dependencies, like this: `laszy = { path = "../laszy_rs" }`
2. You can now import `laszy` in your Rust code, like this: `use laszy::PointCloudBuilder;`

### Using `laszy`:
Laszy uses the builder pattern to create a `PointCloudBuilder` object. This object can be used to lazily process a las/laz file. 
The builder has methods for setting the following parameters:
1. `with_crop`: Crop the point cloud to a bounding box defined by lower left and upper right coordinates.
2. `with_thinning`: Thin the point cloud by only keeping a subset of points. (In the Python bindings this is split into
several methods, in Rust it's one method with a `ThinningMethod` enum.)
3. `with_csf_ground_reclassification`: Reclassify ground points using the CSF algorithm.

Finally the builder has several `to_*` methods to run the builder to a specific output. The following output types are supported:
1. `to_dtm_using_csf`: Create a DTM using the CSF algorithm. This does use the crop and thinning configuration, but ignores the
reclassification configuration.
2. `to_cloud`: Outputs an instance of the `PointCloud` struct/class. Currently does not have many methods or attributes.
3. `to_file`: Outputs a las/laz file, with the same configuration as the input file.

##### As Python module:
```python
from laszy import PointCloudBuilder

# Create a new PointCloudBuilder
builder = PointCloudBuilder.from_file("path/to/file.las")
# Set it's configuration. It's not necessary to reassign the builder to the same variable, as it is mutable 
# and the methods return a reference to the builder.
builder.with_crop((0, 0), (100, 100))
builder.with_thinning_random(0.1)
builder.with_csf_ground_reclassification(0.5, 1.0, 0.01, 1.0)
# Run the builder to a PointCloud
cloud = builder.to_cloud()
# Run the builder to a DTM
builder.to_dtm_using_csf("output.asc", 0.5, 1.0, 0.01)
# Run the builder to a las/laz file
builder.to_file("path/to/output.las")
```

##### As Rust library crate:
```rust
use laszy::{PointCloudBuilder, ThinningMethod, CroppingMethod};

let crop = CroppingMethod::BoundingBox{
    lower_left: (0.0, 0.0),
    upper_right: (100.0, 100.0)
    };

let path = "path/to/file.las".to_string();

// Create a new PointCloudBuilder
let builder = PointCloudBuilder::from_file(&path)
    .unwrap()
    .with_crop(crop)
    .with_thinning(ThinningMethod::Random{percent: 0.1})
    .with_csf_ground_reclassification(0.5, 1.0, 0.01, 1.0);

// Run the builder to a PointCloud
let cloud = builder.to_cloud().unwrap();
```

### `laszy` performance:
The focus of `laszy` is on lazy processing of the files in order to minimize memory usage. Most operations only 
require a single pass over the file, and the only exception is the CSF algorithm, which requires two passes.
Therefore, the performance of `laszy` is heavily dependent on the size of the input file and the speed of the disk.

For reading and writing las/laz files, `laszy` uses the [las](https://crates.io/crates/las) crate. This crate is
written in pure Rust. Currently the heaviest operation is writing the output file.

