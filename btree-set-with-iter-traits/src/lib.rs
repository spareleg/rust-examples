use std::mem;

#[derive(Default, Debug, Clone)]
pub enum BTreeSet<T> {
    #[default]
    Empty,
    Node(Box<Node<T>>),
}

#[derive(Default, Debug, Clone)]
pub struct Node<T> {
    value: T,
    left: BTreeSet<T>,
    right: BTreeSet<T>,
}

impl<T: Ord> BTreeSet<T> {
    pub fn add(&mut self, value: T) {
        match self {
            Self::Empty => {
                *self = Self::Node(Box::new(Node {
                    value,
                    left: Self::Empty,
                    right: Self::Empty,
                }));
            }
            Self::Node(n) if n.value < value => n.right.add(value),
            Self::Node(n) if n.value > value => n.left.add(value),
            _ => {} // `value` is already in the tree
        }
    }

    fn pop(&mut self, left: bool) -> Option<T> {
        match self {
            Self::Empty => None,
            // Delegate if previous node is present
            Self::Node(n) if left && matches!(n.left, Self::Node(_)) => n.left.pop(true),
            Self::Node(n) if !left && matches!(n.right, Self::Node(_)) => n.right.pop(false),
            // The current node is the first
            Self::Node(n) => {
                // Give away current node value and replace current node with the next one
                let mut tree = Self::Empty;
                if left {
                    mem::swap(&mut tree, &mut n.right);
                } else {
                    mem::swap(&mut tree, &mut n.left);
                }
                mem::swap(&mut tree, self);
                let Self::Node(n) = tree else {
                    panic!("impossible: tree comes from self which matched Self::Node pattern")
                };
                Some(n.value)
            }
        }
    }

    pub fn pop_left(&mut self) -> Option<T> {
        self.pop(true)
    }

    pub fn pop_right(&mut self) -> Option<T> {
        self.pop(false)
    }
}

impl<T: Ord> FromIterator<T> for BTreeSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = Self::Empty;
        tree.extend(iter);
        tree
    }
}

impl<T: Ord> Extend<T> for BTreeSet<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.add(item);
        }
    }
}

impl<T: Ord> Iterator for BTreeSet<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.pop_left()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T: Ord> DoubleEndedIterator for BTreeSet<T> {
    fn next_back(&mut self) -> Option<T> {
        self.pop_right()
    }
}

impl<T: Ord> ExactSizeIterator for BTreeSet<T> {
    fn len(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Node(n) => n.left.len() + 1 + n.right.len(),
        }
    }
}

#[test]
fn it_works() {
    let mut tree: BTreeSet<_> = ["zzz", "value", "monkey", "zzz"].into_iter().collect();
    tree.extend(["another", "monkey"]);

    // Only unique values
    assert_eq!(tree.len(), 4);

    assert!(tree.clone().eq(["another", "monkey", "value", "zzz"]));
    assert!(tree.rev().eq(["zzz", "value", "monkey", "another"]));
}
