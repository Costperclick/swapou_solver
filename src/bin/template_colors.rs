use color_thief::{Color, ColorFormat, get_palette};
use screenshots::image::DynamicImage;
use std::{fs::DirEntry, path::PathBuf};

pub enum CellType {
    RedApple,
    GreenPeer,
    OrangeOrange,
    GreenMine,
    RedMine,
    YellowMine,
    GreenStar,
    YellowStar,
    RedStar,
    GreenCrystal,
    YellowCrystal,
    RedCrystal,
    Empty,
}

fn main() {
    const TEMPLATE_DIR: &str = "/home/louis/Projects/swapou_solver/templates/fruit_templates";

    let dir: Vec<DirEntry> = std::fs::read_dir(TEMPLATE_DIR)
        .unwrap()
        .map(|entry| entry.unwrap())
        .collect();

    for entry in dir.iter() {
        let color = get_color(&entry);
        let true_color = color[0];
        let name = entry.file_name().to_string_lossy().into_owned();
        println!("{:?} => {:?}", name, true_color);
    }
}

pub fn get_color(entry: &DirEntry) -> Vec<color_thief::Color> {
    let img = image::open(entry.path()).unwrap();
    let rgb = img.to_rgb8();
    let pixels = rgb.as_raw();

    let color = color_thief::get_palette(pixels, ColorFormat::Rgb, 1, 2).unwrap();
    color
}
