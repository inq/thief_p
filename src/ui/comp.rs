use ui::prim::Buffer;

pub trait Component {
    fn resize(&mut self, width: usize, height: usize);
}

pub struct Child<T: Component> {
    pub comp: T,
    pub x: usize,
    pub y: usize,
}

pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

pub struct Response {
    pub draw: Option<Buffer>,
    pub cursor: Option<Cursor>,
}
