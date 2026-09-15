use color_thief::{ColorFormat, get_palette};
use std::fs::DirEntry;

fn main() {
    read_palette("/home/louis/Projects/swapou_solver/test_img/gen_templates")
}

pub fn read_palette(dir_path: &str) {
    let dir: Vec<DirEntry> = std::fs::read_dir(dir_path)
        .unwrap()
        .map(|entry| entry.unwrap())
        .collect();

    for entry in dir.iter() {
        let output = get_color(entry);
        println!("{:?}", output)
    }
}

pub fn get_color(entry: &DirEntry) -> Vec<color_thief::Color> {
    let img = image::open(entry.path()).unwrap();
    let rgb = img.to_rgb8();
    let pixels = rgb.as_raw();

    let palette = color_thief::get_palette(pixels, ColorFormat::Rgb, 1, 4).unwrap();
    palette
}

pub fn euclidian_distance(item_color:ColorFormat::Rgb) {

    struct ReferenceTemplates {
        green_peer: ColorFormat,
        red_apple: ColorFormat,
        orange_orange: ColorFormat,
        green_mine: ColorFormat,
        red_mine: ColorFormat,
        yellow_mine:ColorFormat,
        green_star:ColorFormat,
        yellow_star:ColorFormat,
        red_star:ColorFormat,
        green_crystal:ColorFormat,
        yellow_crystal:ColorFormat,
        red_crystal:ColorFormat,
    }

    let r = item_color.r;
    let g = item_color.g;
    let b = item_color.b;

    distance =
}
