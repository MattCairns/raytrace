pub struct Screen {
    pub aspect_ratio: f64,
    pub width: u16,
    pub height: u16,
}

impl Screen {
    pub fn new(aspect_ratio: f64, width: u16) -> Self {
        Self {
            aspect_ratio,
            width,
            height: (width as f64 / aspect_ratio) as u16,
        }
    }
}

pub struct ViewPort {
    pub height: f64,
    pub width: f64,
    pub focal_length: f64,
}

impl ViewPort {
    pub fn new(height: f64, width: f64, focal_length: f64) -> Self {
        Self {
            height,
            width,
            focal_length,
        }
    }
}
