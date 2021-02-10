pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More,
}

struct Node {
    elem: i32,
    next: Link,
}