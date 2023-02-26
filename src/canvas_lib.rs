use crate::{colors_lib::Color, constants::PBB_COLOR_COMPONENT_MAX};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<Color>>,
}

/// Converts a 0<=x<=1.0 float to a 0<=x<=255 u8.
fn pbb_float_to_component(x: f64) -> u8 {
    if x > 1.0 {
        PBB_COLOR_COMPONENT_MAX
    } else if x < 0.0 {
        0
    } else {
        (x * PBB_COLOR_COMPONENT_MAX as f64).round() as u8
    }
}

/// A canvas for writing pixels to.
impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut pixels = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(Color::new(0.0, 0.0, 0.0));
            }
            pixels.push(row);
        }
        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y][x] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y][x]
    }

    /// Generates a PPM string representation of the canvas.
    ///
    /// A document in the PPM format will look like this:
    ///
    /// ```ppm
    /// P3             # PPM format identifier
    /// 3 2            # width and height
    /// 255            # maximum color component value
    /// R1 G1 B1 R2 G2 # data section, starting with first row
    /// B2 R3 G3 B3    # still first row, but on a new line
    /// R4 G4 B4 R5 G5 # second row
    /// B5 R6 G6 B6
    /// ```
    ///
    /// The data section of the PPM string consists of lines of up to 70
    /// characters. Each row of the image ends in an additional newline.
    pub fn to_ppm(&self) -> String {
        let mut out = String::new();
        // write header
        out.push_str("P3\n");
        out.push_str(&format!("{} {}\n", self.width, self.height));
        out.push_str(&format!("{}\n", PBB_COLOR_COMPONENT_MAX));

        // write rows into temp objects
        let mut data_rows = vec![];
        for color_row in &self.pixels {
            let mut data_row: Vec<u8> = vec![];
            for pixel in color_row {
                data_row.push(pbb_float_to_component(pixel.red));
                data_row.push(pbb_float_to_component(pixel.green));
                data_row.push(pbb_float_to_component(pixel.blue));
            }
            data_rows.push(data_row);
        }

        let mut line = String::new();
        let write_and_clear_line = |out: &mut String, line: &mut String| {
            out.push_str(&line.trim_end());
            out.push_str("\n");
            line.clear();
        };
        for row in data_rows {
            for color_component in row {
                let color_component_str = format!("{} ", color_component);
                if line.len() + color_component_str.len() > 70 {
                    write_and_clear_line(&mut out, &mut line);
                }
                line.push_str(&color_component_str);
            }
            write_and_clear_line(&mut out, &mut line);
        }
        return out;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for row in c.pixels {
            for pixel in row {
                assert_eq!(pixel, Color::new(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn writing_to_canvas_is_persistent() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn to_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        let lines: Vec<&str> = ppm.split("\n").collect();
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "255");
    }

    #[test]
    fn to_ppm_data() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0., 1.);
        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);

        let ppm = canvas.to_ppm();
        let ppm_lines = ppm.split("\n").collect::<Vec<&str>>();
        assert_eq!(ppm_lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(ppm_lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(ppm_lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn to_ppm_lines_are_below_70() {
        let canvas = Canvas::new(100, 2);
        let ppm = canvas.to_ppm();
        let ppm_lines = ppm.split("\n").collect::<Vec<&str>>();
        for line in ppm_lines {
            assert!(line.len() <= 70);
        }
    }

}