mod grid_reader;
mod screenshot;

fn main() {
    let screenshot = screenshot::take_screenshot();
    let grid = grid_reader::read_grid(&screenshot);
}
