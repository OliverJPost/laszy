use laszy::{CroppingMethod, PointCloudBuilder, ThinningMethod};

fn main() {
    let las_file = String::from("/Users/ole/Downloads/C_68DN1.LAZ");
    let mut builder = PointCloudBuilder::from_file(&las_file).unwrap();
    let mut ptcloud = builder
        .with_crop(CroppingMethod::BoundingBox {
            lower_left: (182011.3, 335505.4),
            upper_right: (182997.8, 336497.5),
        })
        //.with_thinning(ThinningMethod::EveryNth { nth: 40 })
        //.to_dtm_using_csf(&String::from("test17_0.asc"), 0.0, 5.0, 0.1);
        //.to_cloud()
        .perform_csf_reclassification(0.5, 5.0, 0.1)
        .unwrap();
    println!("Cloud has {} points", ptcloud.len());
    let re = ptcloud.to_las(&String::from("actual_tile_csf.las"));
    println!("Result: {:?}", re);
}
