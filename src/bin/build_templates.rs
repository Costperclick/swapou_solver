use image::GenericImageView;
use imageproc::definitions::Image;

fn main() {
    let path = "/home/louis/Projects/swapou_solver/test_img/main_frame_test.png";
    build_templates(path);
}

pub fn build_templates(path: &str) {
    let x_centers = vec![57, 124, 191, 258, 325, 392, 460, 527, 594, 661, 728, 795];
    let y_centers = vec![
        9, 76, 143, 211, 278, 345, 412, 480, 547, 614, 681, 749, 816, 883,
    ];

    let img = image::open(path).unwrap();

    for (i, &cx) in x_centers.iter().enumerate() {
        for (j, &cy) in y_centers.iter().enumerate() {
            let size: i32 = 67 / 2;
            let x = *&cx - size;
            let y = *&cy - size;

            if x < 0 || y < 0 {
                continue;
            }

            let tmp_img = &img.crop_imm(x as u32, y as u32, 67, 67);
            tmp_img
                .save(format!("test_img/gen_templates/{x}_{y}_template.png"))
                .unwrap();
            println!("template has been built");
        }
    }
}
