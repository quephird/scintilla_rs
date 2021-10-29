use crate::color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<color::Color>,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Canvas {
        Canvas {
            width: w,
            height: h,
            pixels: vec![color::BLACK; w*h]
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> color::Color {
        self.pixels[x + y*self.height]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, c: color::Color) {
        self.pixels[x + y*self.height] = c;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let canvas = Canvas::new(10, 20);
        assert!(canvas.get_pixel(0, 0).is_equal(color::BLACK));
        assert!(canvas.get_pixel(5, 5).is_equal(color::BLACK));
        assert!(canvas.get_pixel(9, 9).is_equal(color::BLACK));
    }

    #[test]
    fn test_set_pixel() {
        let mut canvas = Canvas::new(10, 20);
        let red = color::Color::new(1., 0., 0.);
        canvas.set_pixel(2, 3, red);
        assert!(canvas.get_pixel(2, 3).is_equal(red));
    }
}