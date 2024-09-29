use rand::Rng;
use raster::{Image, Color};

pub trait Drawable {
    fn draw(&self, img: &mut Image);
    fn color(&self) -> Color; 
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, clr: Color);
}

fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::rgb(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255))
}


//[POINT]//////////////////////////
#[derive(Clone)]
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

impl Drawable for Point {
    fn draw(&self, img: &mut Image) {
        img.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        random_color() 
    }
}

//[LINE]///////////////////////

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
            img.display(x, y, self.color().clone());

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
        random_color() 
    }
}

//[TRIANGLE]//////////////////

pub struct Triangle {
    pt_a: Point,
    pt_b: Point,
    pt_c: Point,
}

impl Triangle {
    pub fn new(pt_a: Point, pt_b: Point, pt_c: Point) -> Self {        
        Triangle { pt_a, pt_b, pt_c }
    }
}
// pour triangle étant donné qu'on a trois point on les joint avec 3 lignes 
impl Drawable for Triangle {
    fn draw(&self, img: &mut Image) {
        Line::new(self.pt_a.clone(), self.pt_b.clone()).draw(img);
        Line::new(self.pt_b.clone(), self.pt_c.clone()).draw(img);
        Line::new(self.pt_c.clone(), self.pt_a.clone()).draw(img);
    }

    fn color(&self) -> Color {
        random_color()
    }
}

///[RECTANGLE]///////////////////
pub struct Rectangle {
    pt_a: Point,
    pt_b: Point,
}

impl Rectangle {
    pub fn new(pt_a: &Point, pt_b: &Point) -> Self {
        Rectangle {
            pt_a: pt_a.clone(),
            pt_b: pt_b.clone(),
        }
    }
}

impl Drawable for Rectangle {
    // on creer quatre point a b c d doit les extrémié du rectangle et avec Line on relie les points formant ainsi notre Rectangle
    fn draw(&self, img: &mut Image) {
        let pt_c = Point::new(self.pt_a.x, self.pt_b.y);
        let pt_d = Point::new(self.pt_b.x, self.pt_a.y);

        Line::new(self.pt_a.clone(), pt_c.clone()).draw(img);
        Line::new(pt_c.clone(), self.pt_b.clone()).draw(img);
        Line::new(self.pt_b.clone(), pt_d.clone()).draw(img);
        Line::new(pt_d.clone(), self.pt_a.clone()).draw(img);
    }

    fn color(&self) -> Color {
        random_color()
    }
}

///[CIRCLE]////////////////////

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Circle {
            center: center.clone(),
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Circle {
        let center = Point::random(width, height);
        let radius = rand::thread_rng().gen_range(1..=50);
        Circle::new(&center, radius)
    }
}
// Pour creer le cercle on relie tous les points à l'interieur  d'un carré centré sur le centre du cercle
// et on vérifie si chaque point est à l'intérieur du cercle en utilisant l'équation de la distance au carré.
impl Drawable for Circle {
    fn draw(&self, img: &mut Image) {
        let color = self.color();
        for x in -self.radius..=self.radius {
            for y in -self.radius..=self.radius {
                if x * x + y * y <= self.radius * self.radius {
                    img.display(self.center.x + x, self.center.y + y, color.clone());
                }
            }
        }
    }

    fn color(&self) -> Color {
        random_color()
    }
}


///[PENTAGON]////////////////////
pub struct Pentagon {
    points: Vec<Point>,
}

impl Pentagon {
    pub fn new(center: &Point, radius: i32) -> Self {
        let mut points = vec![];
        let angle_step = std::f64::consts::PI * 2.0 / 5.0;

        for i in 0..5 {
            let angle = angle_step * i as f64;
            let x = center.x + (radius as f64 * angle.cos()) as i32;
            let y = center.y + (radius as f64 * angle.sin()) as i32;
            points.push(Point::new(x, y));
        }

        Pentagon { points }
    }

    pub fn random(width: i32, height: i32) -> Pentagon {
        let center = Point::random(width, height);
        let radius = rand::thread_rng().gen_range(30..=100);
        Pentagon::new(&center, radius)
    }
}

impl Drawable for Pentagon {
    fn draw(&self, img: &mut Image) {
        let mut points = self.points.clone();
        points.push(points[0].clone()); 

        for i in 0..points.len() - 1 {
            Line::new(points[i].clone(), points[i + 1].clone()).draw(img);
        }
    }

    fn color(&self) -> Color {
        random_color()
    }
}

