use rand::Rng;
use raster::{Image, Color};

pub trait Drawable {
    fn draw(&self, img: &mut Image);
    fn color(&self) -> Color; // Retourne une couleur aléatoire pour chaque forme
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, clr: Color);
}

///////// // for _ in 1..50 {
    //     gs::Circle::random(image.width, image.height).draw(&mut image);
    // }
///////////////////////////////////////////////////////////////////////////////

pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn random(width: i32, height: i32) -> Point {
        let x = rand::thread_rng().gen_range(0..width);
        let y = rand::thread_rng().gen_range(0..height);

        Point::new(x, y)
    }
}

// j'ai choisi rouge pour creer aléatoirement les point
impl Drawable for Point {
    fn draw(&self, img: &mut Image) {
        img.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        Color::rgb(255, 0, 0) 
    }
}

////////////////////////////////////////////////////////////////////////////////////////

pub struct Line {
    pt_a: Point,
    pt_b: Point,
}

impl Line {
    pub fn new(pt_a: Point, pt_b: Point) -> Line {
        Line { pt_a, pt_b }
    }


    pub fn random(x: i32, y: i32) -> Line {
        Line::new(Point::random(x, y), Point::random(x, y))
    }

}

impl Drawable for Line {
    fn draw(&self, img: &mut Image) {
        // Pour chaque pixel entre pt_a et pt_b, on affiche la couleur 
        let dx = (self.pt_b.x - self.pt_a.x).abs();
        let dy = -(self.pt_b.y - self.pt_a.y).abs();
        let mut err = dx + dy;
        let mut x = self.pt_a.x;
        let mut y = self.pt_a.y;

        loop {
            img.display(x, y, self.color());

            if x == self.pt_b.x && y == self.pt_b.y {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += if self.pt_a.x < self.pt_b.x { 1 } else { -1 };
            }

            if e2 <= dx {
                err += dx;
                y += if self.pt_a.y < self.pt_b.y { 1 } else { -1 };
            }
        }
    }

    fn color(&self) -> Color {
        Color::rgb(0, 255, 0)
    }
}

/////////////////////////////////////////////////////////////////////////////////

// pub struct Triangle {
//     pt_a: Point,
//     pt_b: Point,
//     pt_c: Point,
// }

// impl Triangle {
//     pub fn new(pt_a: Point, pt_b: Point, pt_c: Point) -> Self {
//         let a = pt_a;
        
//         Triangle { pt_a, pt_b, pt_c }
//     }
// }

// impl Drawable for Triangle {
//     fn draw(&self, _: &mut Image) {
        
//     }

//     fn color(&self) {}
// }

// /////////////////////////////////////////////////////////////////////////////////

// pub struct Rectangle {
//     pt_a: Point,
//     pt_b: Point,
// }

// impl Rectangle {
//     pub fn new(pt_a: Point, pt_b: Point) -> Self {
//         Rectangle { pt_a, pt_b }
//     }

// }

// impl Drawable for Rectangle {
//     fn draw(&self, _: &mut Image) {}

//     fn color(&self) {}
// }

// ////////////////////////////////////////////////////////////////////////////////////

// pub struct Circle {
//     center: Point,
//     radius: i32,
// }

// impl Circle {
//     pub fn new(center: Point, radius: i32) -> Self {
//         Circle { center, radius }
//     }

//     pub fn random(x: i32, y: i32) -> Circle {
//         let radius = rand::thread_rng().gen_range(1..=y);
//         Circle::new(Point::new(x, y), radius)
//     }
// }

// impl Drawable for Circle {
//     fn draw(&self, _: &mut Image) {

//     }
    
//     fn color(&self) {

//     }
// }
