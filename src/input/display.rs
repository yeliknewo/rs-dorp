use math::{Vec2};

pub struct Display {
    resolution: Vec2,
    aspect_ratio: f32,
}

impl Display {
    
    pub fn new(resolution: Vec2) -> Display {
        Display {
            aspect_ratio: resolution[0] / resolution[1],
            resolution: resolution,
        }
    }

    
    pub fn set_resolution(&mut self, resolution: Vec2) {
        self.resolution = resolution;
        self.aspect_ratio = resolution[0] / resolution[1];
    }

    
    pub fn get_resolution(&self) -> Vec2 {
        self.resolution
    }

    
    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
}
