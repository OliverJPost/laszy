use laszy::{CroppingMethod, PointCloudBuilder, ThinningMethod};

fn main() {
    let las_file = String::from("/Users/ole/Downloads/C_30GZ2_cropped.las");
    let mut builder = PointCloudBuilder::from_file(&las_file).unwrap();
    let re = builder
        // .with_crop(CroppingMethod::BoundingBox {
        //     lower_left: (182011.3, 335505.4),
        //     upper_right: (182997.8, 336497.5),
        // })
        //.with_thinning(ThinningMethod::EveryNth { nth: 40 })
        //.to_dtm_using_csf(&String::from("test17_0.asc"), 0.0, 5.0, 0.1);
        .with_csf_ground_reclassification(0.5, 5.0, 0.1)
        .to_file(&String::from("result.las"));
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
        .with_csf_ground_reclassification(0.5, 5.0, 0.1)
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
        .with_csf_ground_reclassification(0.5, 5.0, 0.1)
        .to_file(&String::from("result.las"));
    assert!(re.is_ok());
    println!("Result: {:?}", re);
}
