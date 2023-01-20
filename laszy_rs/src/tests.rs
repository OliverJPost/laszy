use crate::{CroppingMethod, PointCloudBuilder, ThinningMethod};

fn get_test_builder() -> PointCloudBuilder {
    let path = "test.las".to_string();
    PointCloudBuilder::from_file(&path).unwrap()
}

#[test]
fn test_incorrect_crop() {
    let mut builder = get_test_builder();
    let re = builder
        .with_crop(CroppingMethod::BoundingBox {
            lower_left: (182_011.3, 335_505.4),
            upper_right: (182_997.8, 336_497.5),
        })
        .with_thinning(ThinningMethod::EveryNth { nth: 40 })
        .with_csf_ground_reclassification(0.5, 5.0, 0.1, 1.0)
        .to_file(&String::from("result.las"));
    assert!(re.is_err());
    println!("Result: {:?}", re);
}

#[test]
fn test_extreme_thinning() {
    let mut builder = get_test_builder();
    let crop = CroppingMethod::BoundingBox {
        lower_left: (183_551.47, 332_414.45),
        upper_right: (183_564.09, 332_424.13),
    };
    let re = builder
        .with_crop(crop)
        .with_thinning(ThinningMethod::EveryNth { nth: 40_000_000 })
        .with_csf_ground_reclassification(0.5, 5.0, 0.1, 1.0)
        .to_file(&String::from("result.las"));
    assert!(re.is_ok());
    println!("Result: {:?}", re);
}

#[test]
fn test_to_file() {
    let mut builder = get_test_builder();
    let crop = CroppingMethod::BoundingBox {
        lower_left: (183_551.47, 332_414.45),
        upper_right: (183_564.09, 332_424.13),
    };
    let re = builder
        .with_crop(crop)
        .with_thinning(ThinningMethod::EveryNth { nth: 40 })
        .with_csf_ground_reclassification(0.5, 5.0, 0.1, 1.0)
        .to_file(&String::from("result.las"));
    assert!(re.is_ok());
    println!("Result: {:?}", re);
}
