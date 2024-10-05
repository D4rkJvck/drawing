mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    gs::Line::random(image.width, image.height).draw(&mut image);

    gs::Point::random(image.width, image.height).draw(&mut image);

    let rectangle = gs::Rectangle::new(&gs::Point::new(250, 350), &gs::Point::new(50, 50));
    rectangle.draw(&mut image);

    let triangle = gs::Triangle::new(
        gs::Point::new(500, 0),
        gs::Point::new(0, 800),
        gs::Point::new(1000, 800),
    );
    triangle.draw(&mut image);

    let pentagon = gs::Pentagon::random(image.width, image.height);
    pentagon.draw(&mut image);

    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    let cube = gs::Cube::new(
        gs::Point::new(350, 350),
        gs::Point::new(450, 450),
        250
    );
    cube.draw(&mut image);

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
