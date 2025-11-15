#[derive(Default, Debug)]
pub enum Tree<T> {
    #[default]
    Empty,
    Node(Box<Node<T>>),
}

#[derive(Default, Debug)]
pub struct Node<T> {
    value: T,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T: Default + Ord> Tree<T> {
    pub fn add(&mut self, value: T) {
        match self {
            Tree::Empty => {
                *self = Tree::Node(Box::new(Node {
                    value,
                    ..Default::default()
                }));
            }
            Tree::Node(n) if n.value < value => {
                n.right.add(value);
            }
            Tree::Node(n) if n.value > value => {
                n.left.add(value);
            }
            _ => {
                // `value` is already in the tree
            }
        }
    }
}

impl<T> Tree<T> {
    pub fn is_empty(&self) -> bool {
        matches!(self, Tree::Empty)
    }

    pub fn len(&self) -> usize {
        match self {
            Tree::Empty => 0,
            Tree::Node(n) => n.left.len() + 1 + n.right.len(),
        }
    }

    fn into_vec(self, v: &mut Vec<T>) {
        if let Tree::Node(n) = self {
            n.left.into_vec(v);
            v.push(n.value);
            n.right.into_vec(v);
        }
    }
}

impl<T> From<Tree<T>> for Vec<T> {
    fn from(t: Tree<T>) -> Self {
        let mut v = Vec::with_capacity(t.len());
        t.into_vec(&mut v);
        v
    }
}

#[test]
fn it_works() {
    let mut tree = Tree::Empty;
    assert!(tree.is_empty());
    tree.add("zzz");
    tree.add("monkey");
    tree.add("value");
    tree.add("another");
    tree.add("monkey");
    tree.add("zzz");
    assert!(!tree.is_empty());
    assert_eq!(tree.len(), 4);
    assert_eq!(Vec::from(tree), vec!["another", "monkey", "value", "zzz"])
}
