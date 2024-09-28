mod geometricatl_shapes

pub trait Drawable {
    fn draw(&mut self)
    fn color(&self)
}

pub trait Displayable {
    fn display(&self)
}