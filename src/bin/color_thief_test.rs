use color_thief::{Color, ColorFormat, get_palette};
use imageproc::template_matching::MatchTemplateMethod;
use std::{fs::DirEntry, path::PathBuf};

pub struct Template {
    name: String,
    path: PathBuf,
    color: Vec<Color>, // will hold 2 most dominant colors.
}

pub struct GridLocation {
    x: char,
    y: i32,
}

pub struct Candidate {
    name: String,
    path: PathBuf,
    color: Vec<Color>, // will hold 2 most dominant colors.
    reference: Option<String>,
    location: Option<GridLocation>,
}

fn main() {
    let templates = get_template(); // Gen les templates de référence.
    let candidates = get_candidates(); // Gen les images à évaluer par rapport aux templates

    for (idx, template) in templates.iter().enumerate() {
        println!("Template n°: {idx} : {:?}", template.color);
    }

    for (idx, candidate) in candidates.iter().enumerate() {
        println!("Candidate n°: {idx} : {:?}", candidate.color);
    }
}

pub fn _crop_candidates(img_path: &str, idx: u32, x: u32, y: u32) -> GridLocation {
    let img = image::open(img_path).unwrap();

    let crop = &img.crop_imm(x, y, 67, 67);
    crop.save(format!("{idx}_{x}_{y}_candidate.png")).unwrap();

    GridLocation { x, y }
}

pub fn get_template() -> Vec<Template> {
    let mut reference = Vec::new();
    let templates_path = "/home/louis/Projects/swapou_solver/templates/fruit_templates";
    let template_dir: Vec<DirEntry> = std::fs::read_dir(templates_path) // Todo : passer le path en constante config
        .unwrap()
        .map(|entry| entry.unwrap())
        .collect();

    for entry in template_dir.iter() {
        let name: String = entry.file_name().to_string_lossy().into_owned();
        let path = entry.path();
        let color = get_color(entry);

        let tmp_template = Template { name, path, color };
        reference.push(tmp_template);
    }
    reference // To do : add missing stars and check if the vec length is enought.
}

pub fn get_candidates() -> Vec<Candidate> {
    let mut candidates = Vec::new();
    let candidates_path = "/home/louis/Projects/swapou_solver/test_img/gen_templates";
    let dir: Vec<DirEntry> = std::fs::read_dir(candidates_path)
        .unwrap()
        .map(|entry| entry.unwrap())
        .collect();

    for entry in dir {
        let name: String = entry.file_name().to_string_lossy().into_owned();
        let path = entry.path();
        let color = get_color(&entry);

        let tmp_candidate = Candidate {
            name,
            path,
            color,
            reference: None,
            location: None,
        };
        candidates.push(tmp_candidate);
    }
    candidates
}

pub fn get_color(entry: &DirEntry) -> Vec<color_thief::Color> {
    let img = image::open(entry.path()).unwrap();
    let rgb = img.to_rgb8();
    let pixels = rgb.as_raw();

    let palette = color_thief::get_palette(pixels, ColorFormat::Rgb, 1, 2).unwrap();
    palette
}

pub fn compute_distance(item_color: &Color, template: &Color) -> f32 {
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

pub fn compare_distance(item_color: &Color, templates: &Vec<&Color>) -> usize {
    let mut closest_idx: usize = 0;
    let mut closest_value: f32 = 1000.0;

    for (idx, template) in templates.iter().enumerate() {
        let r = compute_distance(item_color, template);
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

pub fn find_block_type(candidates: Vec<Candidate>, references: Vec<Template>) {
    let mut template_colors = Vec::new();

    for t in references.iter() {
        template_colors.push(&t.color[0])
    }

    for (idx, candidate) in candidates.iter().enumerate() {
        let matching_template_idx = compare_distance(&candidate.color[0], &template_colors);
        candidate.reference = Some(references[matching_template_idx].name.clone());
    }
}
