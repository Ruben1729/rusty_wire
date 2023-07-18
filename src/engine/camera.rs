pub struct Camera {
    coordinates: (usize, usize),
    dimensions: (usize, usize),
    direction: (usize, usize),
    velocity: f64,
    scale: usize
}

impl Camera {
    pub fn new(coordinates: (usize, usize),
               dimensions: (usize, usize),
               scale: usize,
               velocity: f64) -> Self {
        Camera {
            coordinates,
            dimensions,
            direction: (0, 0),
            velocity,
            scale
        }
    }

    pub fn scale(&self) -> usize {
        self.scale
    }

    pub fn set_direction(&mut self, dx: usize, dy: usize) {
        self.direction.0 = dx;
        self.direction.1 = dy;
    }

    pub fn zoom_in(&mut self) {
        if self.scale < 10 {
            self.scale += 1;
        }
    }

    pub fn zoom_out(&mut self) {
        if self.scale > 1 {
            self.scale -= 1;
        }
    }

    pub fn update(&mut self) {
        self.coordinates.0 += self.direction.0 * self.velocity as usize;
        self.coordinates.1 += self.direction.1 * self.velocity as usize;
    }

    pub fn render<F: FnMut(usize, usize, usize)>(&mut self, mut operation: F) {
        for x in self.coordinates.0..(self.coordinates.0 + self.dimensions.0) {
            for y in self.coordinates.1..(self.coordinates.1 + self.dimensions.1) {
                operation(x, y, self.scale);
            }
        }
    }
}
