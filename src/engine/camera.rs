pub struct Camera {
    coordinates: (f64, f64),
    velocity: (f64, f64),
    pixel_size: usize
}

impl Camera {
    pub fn new(coordinates: (f64, f64),
                pixel_size: usize) -> Self {
        Camera {
            coordinates,
            velocity: (0.0, 0.0),
            pixel_size
        }
    }

    pub fn update(&mut self) {
        self.coordinates.0 += self.velocity.0;
        self.coordinates.1 += self.velocity.1;
    }

    pub fn render(&mut self) {

    }
}
