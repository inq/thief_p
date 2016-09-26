use ui::prim::Buffer;

pub trait Component {
    fn resize(&mut self, width: usize, height: usize);
    fn refresh(&self) -> Response;
}

pub trait Parent<T: Component> {
    fn children_mut(&mut self) -> Vec<&mut Child<T>>;
    fn children(&self) -> Vec<&Child<T>>;

    fn refresh_children(&self, mut buffer: Buffer) -> Response {
        let mut cursor = None;
        for &ref child in self.children() {
            let r = child.comp.refresh();
            if let Some(buf) = r.draw {
                buffer.draw(&buf, child.x, child.y);
            }
            if let Some(cur) = r.cursor {
                cursor = Some(Cursor { x: cur.x + child.x, y: cur.y + child.y } );
            }
        }
        Response {
            draw: Some(buffer),
            cursor: cursor,
        }
    }
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
