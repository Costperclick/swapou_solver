use color_thief::{Color, ColorFormat, get_palette};
use screenshots::image::DynamicImage;
use std::{fs::DirEntry, path::PathBuf};

pub struct Grid {
    candidates: Vec<Candidate>,
    cols: usize,
    rows: usize,
}

impl Grid {
    pub fn get(&self, col: usize, row: usize) -> Option<&Candidate> {
        self.candidates.get(row * self.cols + col)
    }

    pub fn display(&self) {
        for row in 0..self.cols - 1 {
            for col in 0..self.cols - 1 {
                let cell = self.get(col, row);
                let to_print = cell.template_reference;
                print!("{cell.")
            }
        }
    }
}

pub struct Template {
    name: String,
    path: PathBuf,
    color: Vec<Color>, // will hold 2 most dominant colors.
}

pub struct GridConfig {
    x_origin: i32,
    y_origin: i32,
    x_step: i32,
    y_step: i32,
    cell_size: u32,
    cols: usize,
    rows: usize,
}

pub struct GridLocation {
    col: usize,
    row: usize,
}

pub struct Candidate {
    name: String,
    color: Vec<Color>, // will hold 2 most dominant colors.
    template_reference: Option<String>,
    location: GridLocation,
}

pub fn read_grid(screenshot: &DynamicImage) {
    const TEMPLATE_DIR: &str = "/home/louis/Projects/swapou_solver/templates/fruit_templates";
    const ACTUAL_GRID: &str = "/home/louis/Projects/swapou_solver/test_img/gen_templates";

    let grid = GridConfig {
        x_origin: 57,
        y_origin: 9,
        x_step: 67,
        y_step: 67,
        cell_size: 67,
        cols: 12,
        rows: 14,
    };

    // Open each dir and build an associated Vec<Struct> :
    let templates = generate_template(TEMPLATE_DIR);
    let mut candidates = generate_candidates(screenshot, &grid);

    // populate every candidate with their template_reference:
    match_candidate_reference_template(&mut candidates, &templates);
}

fn _get_color(img: &DynamicImage) -> Vec<color_thief::Color> {
    let rgb = img.to_rgb8();
    let pixels = rgb.as_raw();

    let palette = color_thief::get_palette(pixels, ColorFormat::Rgb, 1, 2).unwrap();
    palette
}

fn _compute_distance(item_color: &Color, template: &Color) -> f32 {
    let cir = item_color.r as f32; // "cir = converted item's red"
    let cig = item_color.g as f32;
    let cib = item_color.b as f32;

    let ctr = template.r as f32; // "ctr = converted template's red"
    let ctg = template.g as f32;
    let ctb = template.b as f32;

    // used Eclidian distance in RGB color here : https://observablehq.com/@luciyer/euclidian-distance-in-rgb-color-space
    let distance = ((ctr - cir).powi(2)) + ((ctg - cig).powi(2)) + ((ctb - cib).powi(2));
    let final_distance = distance.sqrt();
    final_distance
}

fn _compare_distance(item_color: &Color, templates: &Vec<&Color>) -> usize {
    let mut closest_idx: usize = 0;
    let mut closest_value: f32 = 1000.0;

    for (idx, template) in templates.iter().enumerate() {
        let r = _compute_distance(item_color, template);
        if idx == 0 {
            closest_value = r
        } else {
            if r < closest_value {
                closest_value = r;
                closest_idx = idx
            };
        }
    }
    closest_idx
}

fn generate_template(path: &str) -> Vec<Template> {
    let mut templates = Vec::new();

    let template_dir: Vec<DirEntry> = std::fs::read_dir(path)
        .unwrap()
        .map(|entry| entry.unwrap())
        .collect();

    for entry in template_dir.iter() {
        let tmp_template = Template {
            name: String::from(entry.file_name().to_string_lossy().into_owned()),
            path: entry.path(),
            color: _get_color(entry),
        };
        templates.push(tmp_template)
    }
    templates
}

pub fn generate_candidates(screenshot: &DynamicImage, grid: &GridConfig) -> Vec<Candidate> {
    let mut candidates = Vec::new();
    let half = grid.cell_size as i32 / 2;

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            // define cell centers :
            let cx = grid.x_origin + col as i32 * grid.x_step;
            let cy = grid.y_origin + row as i32 * grid.y_step;

            // deduct top left corner :
            let x = cx - half;
            let y = cy - half;

            // skip if overextends :
            if x < 0 || y < 0 {
                continue;
            }

            // crop img :
            let crop = &screenshot.crop_imm(x as u32, y as u32, grid.cell_size, grid.cell_size);
            // extract color:

            let color = _get_color(crop);

            // build candidate :
            //

            let _col = col as usize;
            let _row = row as usize;

            candidates.push(Candidate {
                name: format!("{col}_{row}"),
                color: color,
                template_reference: None,
                location: GridLocation { col, row },
            });
        }
    }
    candidates
}

pub fn match_candidate_reference_template(candidates: &mut [Candidate], references: &[Template]) {
    let template_colors_vec: Vec<&Color> = references
        .iter()
        .map(|reference| &reference.color[0])
        .collect();

    for candidate in candidates.iter_mut() {
        let matching_template_idx = _compare_distance(&candidate.color[0], &template_colors_vec);
        let associated_template = references[matching_template_idx].name.clone();
        candidate.template_reference = Some(associated_template);
    }
}
