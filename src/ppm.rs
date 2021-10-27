use std::fs;
use std::fs::File;
use std::io::{Error, Write};

use crate::canvas;
use crate::color;

const MAX_LINE_WIDTH: usize = 70;
const MAX_COLOR_COMPONENT_WIDTH: usize = 3;

fn scale_and_clamp(f: f64) -> u8 {
    if f < 0.0 {
        0
    } else if f >= 1.0 {
        255
    } else {
        (f*256.) as u8
    }
}

pub trait Saveable {
    fn write_header(&self, file: &mut File);

    fn write_color_component(&self, current_line: &mut String, f: f64);

    fn write_separator(&self, file: &mut File, current_line: &mut String);

    fn write_color(&self, file: &mut File, current_line: &mut String, c: color::Color);

    fn write_pixel_row(&self, file: &mut File, y: usize);

    fn write_body(&self, file: &mut File);

    fn save(&self, file_name: &str) -> Result<(), Error>;
}

impl Saveable for canvas::Canvas {
    fn write_header(&self, file: &mut File) {
        write!(file, "P3\n{} {}\n255\n", self.width, self.height).unwrap()
    }

    fn write_color_component(&self, current_line: &mut String, f: f64) {
        current_line.push_str(&scale_and_clamp(f).to_string());
    }

    fn write_separator(&self, file: &mut File, current_line: &mut String) {
        if current_line.len() >= MAX_LINE_WIDTH - MAX_COLOR_COMPONENT_WIDTH {
            write!(file, "{}\n", current_line).unwrap();
            current_line.clear();
        } else {
            current_line.push_str(" ");
        }
    }

    fn write_color(&self, file: &mut File, current_line: &mut String, c: color::Color) {
        self.write_color_component(current_line, c[0]);
        self.write_separator(file, current_line);
        self.write_color_component( current_line, c[1]);
        self.write_separator(file, current_line);
        self.write_color_component( current_line, c[2]);
    }

    fn write_pixel_row(&self, file: &mut File, y: usize) {
        let mut current_line = String::new();
        for x in 0..self.width {
            let c = self.get_pixel(x, y);
            self.write_color(file, &mut current_line, c);
            if x < self.width-1 {
                current_line.push_str(" ");
            }
        }
        write!(file, "{}\n", current_line).unwrap();
    }

    fn write_body(&self, file: &mut File) {
        for y in 0..self.height {
            self.write_pixel_row(file, y);
        }
    }

    fn save(&self, file_name: &str) -> Result<(), Error> {
        let mut file = File::create(file_name)?;
        self.write_header(&mut file);
        self.write_body(&mut file);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_with_tiny_canvas() -> Result<(), Error> {
        let mut canvas = canvas::Canvas::new(5, 3);
        let c1 = [1.5, 0.0, 0.0];
        let c2 = [0.0, 0.5, 0.0];
        let c3 = [-0.5, 0.0, 1.0];
        canvas.set_pixel(0, 0, c1);
        canvas.set_pixel(2, 1, c2);
        canvas.set_pixel(4, 2, c3);

        let test_file_name = "test.ppm";
        canvas.save(test_file_name)?;

        let contents = fs::read_to_string(test_file_name)?;
        let expected_value = "\
P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
";
        assert_eq!(contents, expected_value);
        fs::remove_file(test_file_name)?;
        Ok(())
    }

    #[test]
    fn test_save_splitting_long_lines() -> Result<(), Error> {
        let w = 10;
        let h = 2;
        let mut canvas = canvas::Canvas::new(w, h);

        let c = [1.0, 0.8, 0.6];
        for y in 0..h {
            for x in 0..w {
                canvas.set_pixel(x, y, c);
            }
        }

        let test_file_name = "test2.ppm";
        canvas.save(test_file_name)?;

        let contents = fs::read_to_string(test_file_name)?;
        let expected_value = "\
P3
10 2
255
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
";
        assert_eq!(contents, expected_value);
        fs::remove_file(test_file_name)?;
        Ok(())
    }
}
