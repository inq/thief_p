pub trait Component {
    fn resize(&mut self, width: usize, height: usize);
}

pub struct Child<T: Component> {
    pub comp: T,
    pub x: usize,
    pub y: usize,
}
