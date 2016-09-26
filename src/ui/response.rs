use ui::prim::Buffer;

enum Response {
    Draw(Buffer),
    Cursor { x: usize, y: usize },
}
