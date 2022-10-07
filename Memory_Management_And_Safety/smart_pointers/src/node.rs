#[derive(Debug)]
pub struct Node {
    pub data: u32,
    pub next: Option<Box<Node>>,
}

pub fn node() {
    let node = Node {
        data: 32,
        next: Some(Box::new(Node {
            data: 36,
            next: Some(Box::new(Node {
                data: 37,
                next: None,
            })),
        })),
    };

    println!("{:#?}", node);
}
