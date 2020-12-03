use rand::Rng;

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty =>
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty
                })),
            BinaryTree::NonEmpty(ref mut node) =>
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
        }
    }
}

impl<T: std::fmt::Display> BinaryTree<T> {
    fn print(&self) {
        match *self {
            BinaryTree::NonEmpty(ref node) => {
                node.left.print();
                println!("{}", node.element);
                node.right.print();
            },
            BinaryTree::Empty => ()
        }
    }
}

fn main() {
    let mut tree = BinaryTree::Empty;
    let mut rng = rand::thread_rng();
    for _ in 1..100 {
        tree.add(rng.gen::<u8>());
    }
    tree.print();
}
