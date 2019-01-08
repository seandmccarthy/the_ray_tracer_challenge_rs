use super::colour::Colour;

pub struct Canvas {
    width: usize,
    height: usize,
    pub pixels: Vec<Colour>
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        Canvas { width, height, pixels:
            vec![Colour { red: 0.0, green: 0.0, blue: 0.0 }; width * height]
        }
    }

    fn pixel_at(&self, x: usize, y: usize) -> & Colour {
        &self.pixels[x + self.width * y]
    }

    fn write_pixel(&mut self, x: usize, y: usize, colour: Colour) {
        self.pixels[x + self.width * y] = colour;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rt::colour::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(1, 2);
        assert_eq!(c.width, 1);
        assert_eq!(c.height, 2);
    }

    #[test]
    fn get_a_pixel() {
        let c = Canvas::new(1, 2);
        assert_eq!(c.pixel_at(0, 0), &Colour { red: 0.0, green: 0.0, blue: 0.0 })
    }

    #[test]
    fn write_a_pixel() {
        let mut c = Canvas::new(1, 2);
        let red = colour(1.0, 0.0, 0.0);
        c.write_pixel(0, 0, red);
        assert_eq!(c.pixel_at(0, 0), &colour(1.0, 0.0, 0.0));
    }
}
