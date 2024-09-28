pub trait Drawable {
    fn draw(&self);
    fn color(&self);
}

pub trait Displayable {
    fn display(&self);
}