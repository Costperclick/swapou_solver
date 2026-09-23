use screenshots::{Screen, image::DynamicImage};
use std::time::Instant;

struct MainFrame {
    top_left: (i32, i32),
    bottom_right: (i32, i32),
}

impl MainFrame {
    fn x(&self) -> i32 {
        self.top_left.0
    }
    fn y(&self) -> i32 {
        self.top_left.1
    }
    fn width(&self) -> u32 {
        (self.bottom_right.0 - self.top_left.0) as u32
    }
    fn height(&self) -> u32 {
        (self.bottom_right.1 - self.top_left.1) as u32
    }
}

pub fn take_screenshot() -> DynamicImage {
    let _start = Instant::now();
    let screens = Screen::all().unwrap();
    let screen = screens[0];

    let frame = MainFrame {
        top_left: (619, 142),
        bottom_right: (1470, 1064),
    };

    let main_frame = screen
        .capture_area(frame.x(), frame.y(), frame.width(), frame.height())
        .unwrap();

    let screenshot = DynamicImage::ImageRgba8(main_frame);
    screenshot
}
