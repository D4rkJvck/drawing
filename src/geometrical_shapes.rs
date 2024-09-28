pub trait Drawable {
    fn draw(&self);
    fn color(&self);
}

pub trait Displayable {
    fn display(&self);
}

///////////////////////////////////////////////////////////////////////

pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }
}

impl Drawable for Point {   
    fn draw(&self) {
        
    }
    
    fn color(&self) {
        
    }
}

 impl Displayable for Point {   
    fn display(&self) {
        
    }
}

//////////////////////////////////////////////////////////////

pub struct Line {
    pt_a: Point,
    pt_b: Point,
}

impl Line {
    pub fn new(pt_a: Point, pt_b: Point) -> Line {
        Line { pt_a, pt_b }
    }

    pub fn get_pt_a(&self) -> &Point {
        &self.pt_a
    }

    pub fn get_pt_b(&self) -> &Point {
        &self.pt_b
    }
}

impl Drawable for Line {
    fn draw(&self) {

    }

    fn color(&self) {

    }
}

impl Displayable for Line {
    fn display(&self) {

    }
}

////////////////////////////////////////////////////////////////

pub struct Triangle {
    pt_a: Point,
    pt_b: Point,
    pt_c: Point,
}

impl Triangle {
    pub fn new(pt_a: Point, pt_b: Point, pt_c: Point) -> Self {
        Triangle { pt_a, pt_b, pt_c }
    }

    pub fn get_pt_a(&self) -> &Point {
        &self.pt_a
    }

    pub fn get_pt_b(&self) -> &Point {
        &self.pt_b
    }

    pub fn get_pt_c(&self) -> &Point {
        &self.pt_c
    }
}

impl Drawable for Triangle {
    fn draw(&self) {

    }

    fn color(&self) {

    }
}

impl Displayable for Triangle {
    fn display(&self) {

    }
}

////////////////////////////////////////////////////////////////

pub struct Rectangle {
    pt_a: Point,
    pt_b: Point,
}

impl Rectangle {
    pub fn new(pt_a: Point, pt_b: Point) -> Self {
        Rectangle { pt_a, pt_b }
    }

    pub fn get_pt_a(&self) -> &Point {
        &self.pt_a
    }

    pub fn get_pt_b(&self) -> &Point {
        &self.pt_b
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        
    }

    fn color(&self) {
        
    }
}

impl Displayable for Rectangle {
    fn display(&self) {
        
    }
}

///////////////////////////////////////////////////////////////////
 
pub struct Circle {
    center: Point,
    radius: i32
}

impl Circle {
    pub fn new (center: Point, radius: i32) -> Self {
        Circle { center, radius }
    }

    pub fn get_center(&self) -> &Point {
        &self.center
    }

    pub fn get_radius(&self) -> i32 {
        self.radius
    }
}

impl Drawable for Circle {
    fn draw(&self) {
        
    }

    fn color(&self) {
        
    }
}

impl Displayable for Circle {
    fn display(&self) {
        
    }
}