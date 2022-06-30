#[derive(Debug)]
pub enum Tree {
    Node {
        left: Box<Tree>,
        value: i32,
        right: Box<Tree>,
    },
    Leaf(i32),
}

pub fn make() -> Tree {
    return Tree::Node {
        left: Box::new(Tree::Leaf(0)),
        value: 1,
        right: Box::new(Tree::Leaf(2)),
    };
}

pub fn flatten(tree: Tree) -> Vec<i32> {
    match tree {
        Tree::Leaf(value) => Vec::from([value]),
        Tree::Node { left, value, right } => {
            let mut res = Vec::new();

            for x in flatten(*left) {
                res.push(x);
            }

            res.push(value);

            for x in flatten(*right) {
                res.push(x);
            }

            return res;
        }
    }
}

pub fn run() {
    let tree = make();
    println!("{:?}", flatten(tree));
}
