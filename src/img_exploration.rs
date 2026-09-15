use image::Rgba;
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;

struct Centers {
    x_centers: Vec<i32>,
    y_centers: Vec<i32>,
}

pub fn draw_squares(img: &str) {
    let centers = Centers {
        x_centers: vec![57, 124, 191, 258, 325, 392, 460, 527, 594, 661, 728, 795],
        y_centers: vec![
            9, 76, 143, 211, 278, 345, 412, 480, 547, 614, 681, 749, 816, 883,
        ],
    };

    let mut img = image::open(img).unwrap().to_rgba8();
    let red = Rgba([255u8, 0, 0, 255]);
    let size: u32 = 67;
    let half = (size / 2) as i32;

    for &x_coord in &centers.x_centers {
        for &y_coord in &centers.y_centers {
            let top_left_x = x_coord - half;
            let top_left_y = y_coord - half;
            draw_hollow_rect_mut(
                &mut img,
                Rect::at(top_left_x, top_left_y).of_size(size, size),
                red,
            );
            println!("Square drawned");
        }
    }
    img.save("test_img/squared.png").unwrap()
}
