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
    pub fn new(x: i32, y: i32) -> Self {
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
    fn draw(&mut self) {
        
    }
    
    fn color(&self) {
        
    }
}

 impl Displayable for Point {   
    fn display(&self) {
        
    }
}

//////////////////////////////////////////////////////////////

pub struct Line<'a> {
    pt_a: &'a Point,
    pt_b: &'a Point,
}

impl<'a> Line<'a> {
    pub fn new(pt_a: &'a Point, pt_b: &'a Point) -> Self {
        Line { pt_a, pt_b }
    }

    pub fn get_pt_a(&self) -> &Point {
        self.pt_a
    }

    pub fn get_pt_b(&self) -> &Point {
        self.pt_b
    }
}

impl<'a> Drawable for Line<'a> {
    fn draw(&self) {

    }

    fn color(&self) {

    }
}

impl<'a> Displayable for Line<'a> {
    fn display(&self) {

    }
}

////////////////////////////////////////////////////////////////

pub struct Triangle<'a> {
    pt_a: &'a Point,
    pt_b: &'a Point,
    pt_c: &'a Point,
}

impl<'a> Triangle<'a> {
    pub fn new(pt_a: &'a Point, pt_b: &'a Point, pt_c: &'a Point) -> Self {
        Triangle { pt_a, pt_b, pt_c }
    }

    pub fn get_pt_a(&self) -> &Point {
        self.pt_a
    }

    pub fn get_pt_b(&self) -> &Point {
        self.pt_b
    }

    pub fn get_pt_c(&self) -> &Point {
        self.pt_c
    }
}

impl<'a> Drawable for Triangle<'a> {
    fn draw(&self) {

    }

    fn color(&self) {

    }
}

impl<'a> Displayable for Triangle<'a> {
    fn display(&self) {

    }
}

////////////////////////////////////////////////////////////////

pub struct Rectangle<'a> {
    pt_a: &'a Point,
    pt_b: &'a Point,
}

impl<'a> Rectangle<'a> {
    pub fn new(pt_a: &'a Point, pt_b: &'a Point) -> Self {
        Rectangle { pt_a, pt_b }
    }

    pub fn get_pt_a(&self) -> &Point {
        self.pt_a
    }

    pub fn get_pt_b(&self) -> &Point {
        self.pt_b
    }
}

impl<'a> Drawable for Rectangle<'a> {
    fn draw(&self) {
        
    }

    fn color(&self) {
        
    }
}

impl<'a> Displayable for Rectangle<'a> {
    fn display(&self) {
        
    }
}

///////////////////////////////////////////////////////////////////
 
pub struct Circle<'a> {
    center: &'a Point,
    radius: i32
}

impl<'a> Circle<'a> {
    pub fn new (center: &'a Point, radius: i32) -> Self {
        Circle { center, radius }
    }

    pub fn get_center(&self) -> &Point {
        self.center
    }

    pub fn get_radius(&self) -> i32 {
        self.radius
    }
}

impl<'a> Drawable for Circle<'a> {
    fn draw(&self) {
        
    }

    fn color(&self) {
        
    }
}

impl<'a> Displayable for Circle<'a> {
    fn display(&self) {
        
    }
}