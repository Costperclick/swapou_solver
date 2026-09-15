mod build_templates;
mod image_reader;
mod img_exploration;
mod screenshot;

fn main() {
    screenshot::take_screenshot();

    let path = "/home/louis/Projects/swapou_solver/test_img/main_frame_test.png";
    img_exploration::draw_squares(path);
}
