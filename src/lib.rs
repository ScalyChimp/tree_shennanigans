#![allow(dead_code)]

use std::cmp::Ordering;
use std::fmt::Debug;

struct BSTree<T: Eq + Ord + Debug> {
    root: BSTNode<T>,
}
impl<T> BSTree<T>
where
    T: Eq + Ord + Debug,
{
    fn new(data: T) -> BSTree<T> {
        BSTree {
            root: BSTNode {
                data,
                data_count: 1,
                less: None,
                more: None,
            },
        }
    }

    fn insert(&mut self, data: T) {
        self.root.insert(data)
    }

    fn remove(&mut self, data: T) {
        self.root.remove(data)
    }
}

enum Error {
    Err,
}

#[derive(PartialEq, Debug)]
struct BSTNode<T: Eq + Ord + Debug> {
    data: T,
    data_count: u64,

    less: Option<Box<BSTNode<T>>>,
    more: Option<Box<BSTNode<T>>>,
}

impl<T: Eq + Ord + Debug> BSTNode<T> {
    fn insert(&mut self, data: T) {
        match self.data.cmp(&data) {
            Ordering::Greater => match &mut self.less {
                Some(less) => less.insert(data),
                None => self.less = Some(BSTNode::new_boxed(data)),
            },
            Ordering::Equal => {
                self.data_count += 1; // Base case
            }
            Ordering::Less => match &mut self.more {
                Some(more) => more.insert(data),
                None => self.more = Some(BSTNode::new_boxed(data)),
            },
        }
    }

    fn remove(&mut self, data: T) {
        dbg!(&self.data, &data);
        match self.data.cmp(&data) {
            Ordering::Greater => match &mut self.less {
                Some(less) => {
                    if less.data == data {
                        let less = self.less.take();
                        drop(less)
                    } else {
                        less.remove(data);
                    }
                }
                None => return,
            },
            Ordering::Less => match &mut self.more {
                Some(more) => {
                    if more.data == data {
                        let more = self.more.take();
                        drop(more)
                    } else {
                        more.remove(data)
                    }
                }
                None => return,
            },
            Ordering::Equal => {
                // Base case
                panic!("This node should've been removed in an earlier iteration")
            }
        }
    }

    fn new_boxed(data: T) -> Box<BSTNode<T>> {
        Box::new(BSTNode {
            data,
            data_count: 1,
            less: None,
            more: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Debugging utilities
    impl<T: Ord + Debug> BSTNode<T> {
        fn less(&self) -> &BSTNode<T> {
            self.less.as_ref().unwrap()
        }
        fn more(&self) -> &BSTNode<T> {
            self.more.as_ref().unwrap()
        }
    }

    #[test]
    fn level_one_insertion() {
        let mut tree = BSTree::new(1);
        tree.insert(2);
        assert_eq!(tree.root.more().data, 2);
        tree.insert(0);
        assert_eq!(tree.root.less().data, 0);
    }

    #[test]
    fn level_two_insertion() {
        let mut tree = BSTree::new(2);
        tree.insert(1);
        tree.insert(3);

        tree.insert(4);
        tree.insert(0);

        assert_eq!(tree.root.more().more().data, 4);
        assert_eq!(tree.root.less().less().data, 0);
    }

    #[test]
    fn level_one_removal() {
        let mut tree = BSTree::new(2);
        tree.insert(1);
        tree.insert(3);

        assert_eq!(tree.root.more().data, 3);
        assert_eq!(tree.root.less().data, 1);

        tree.remove(1);
        tree.remove(3);

        assert_eq!(tree.root.more, None);
        assert_eq!(tree.root.less, None);
    }
}
