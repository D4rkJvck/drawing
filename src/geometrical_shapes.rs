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
    a: Point,
    b: Point,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Line {
        Line { a, b }
    }


    pub fn random(x: i32, y: i32) -> Line {
        Line::new(Point::random(x, y), Point::random(x, y))
    }

}

impl Drawable for Line {
    fn draw(&self, img: &mut Image) {
        // Pour chaque pixel entre a et b, on affiche la couleur 
        let dx = (self.b.x - self.a.x).abs();
        let dy = -(self.b.y - self.a.y).abs();
        let mut err = dx + dy;
        let mut x = self.a.x;
        let mut y = self.a.y;

        loop {
            img.display(x, y, self.color().clone());

            if x == self.b.x && y == self.b.y {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += if self.a.x < self.b.x { 1 } else { -1 };
            }

            if e2 <= dx {
                err += dx;
                y += if self.a.y < self.b.y { 1 } else { -1 };
            }
        }
    }

    fn color(&self) -> Color {
        random_color() 
    }
}

//[TRIANGLE]//////////////////

pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {        
        Triangle { a, b, c }
    }
}
// pour triangle étant donné qu'on a trois point on les joint avec 3 lignes 
impl Drawable for Triangle {
    fn draw(&self, img: &mut Image) {
        Line::new(self.a.clone(), self.b.clone()).draw(img);
        Line::new(self.b.clone(), self.c.clone()).draw(img);
        Line::new(self.c.clone(), self.a.clone()).draw(img);
    }

    fn color(&self) -> Color {
        random_color()
    }
}

///[RECTANGLE]///////////////////
pub struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    pub fn new(a: &Point, b: &Point) -> Self {
        Rectangle {
            a: a.clone(),
            b: b.clone(),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, img: &mut Image) {
        let c = Point::new(self.b.x, self.a.y);
        let d = Point::new(self.a.x, self.b.y);

        Line::new(self.a.clone(), c.clone()).draw(img);
        Line::new(c.clone(), self.b.clone()).draw(img);
        Line::new(self.b.clone(), d.clone()).draw(img);
        Line::new(d.clone(), self.a.clone()).draw(img);
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
        let mut angle = 0.0;

        while angle < 2.0 * std::f64::consts::PI {
            let x = self.center.x + (self.radius as f64 * angle.cos()) as i32;
            let y = self.center.y + (self.radius as f64 * angle.sin()) as i32;
            img.display(x, y, color.clone());
            angle += 0.01; 
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

