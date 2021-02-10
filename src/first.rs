 struct Node {
    value: i32,
    next: List,
}
pub enum List {
    Empty,
    More(Box<Node>),
}