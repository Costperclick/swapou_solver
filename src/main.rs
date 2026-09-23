mod grid_reader;
mod screenshot;

fn main() {
    //let screenshot = screenshot::take_screenshot();

    let screenshot =
        screenshots::image::open("/home/louis/Projects/swapou_solver/test_img/main_frame_test.png")
            .unwrap();
    let grid = grid_reader::read_grid(&screenshot);

    grid.display()
}
