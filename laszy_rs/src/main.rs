use laszy::{CroppingMethod, PointCloudBuilder};

fn main() {
    let las_file = String::from("/Users/ole/Downloads/C_68DN1_CROPPED1.LAS");
    let mut builder = PointCloudBuilder::from_file(&las_file).unwrap();
    let mut ptcloud = builder
        //.with_crop(CroppingMethod::BoundingBox {
        //    lower_left: (183536.12, 332378.91),
        //   upper_right: (183579.03, 332431.91),
        //})
        .to_dtm_using_csf(&String::from("test14.asc"), 1.0, 5.0, 0.1);
    // .to_cloud();
}
