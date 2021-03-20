use std::mem::{replace, swap};

use super::BBST;

// https://faculty.washington.edu/aragon/pubs/rst96.pdf
// Figure 3
struct Node<K: Ord> {
  key: K,
  priority: u64,
  lchild: Option<Box<Node<K>>>,
  rchild: Option<Box<Node<K>>>,
}

pub struct Treap<K: Ord> {
  root: Option<Box<Node<K>>>,
}

impl<K: Ord> Treap<K> {
  pub fn new() -> Treap<K> {
    Treap { root: None }
  }

  fn _insert(item: Box<Node<K>>, tree: &mut Option<Box<Node<K>>>) {
    match tree {
      None => {
        *tree = Some(item);
      }
      Some(t) => {
        if item.key > t.key {
          Treap::_insert(item, &mut t.lchild);
        } else if item.key < t.key {
          Treap::_insert(item, &mut t.rchild);
        }
      }
    }
  }
}

impl<K: Ord> BBST<K> for Treap<K> {
  fn insert(&mut self, key: K) {
    let item: Box<Node<K>> = Box::new(Node {
      key,
      priority: rand::random(),
      lchild: None,
      rchild: None,
    });
    Treap::_insert(item, &mut self.root);
  }
}

#[cfg(test)]
mod tests {
  use super::Treap;
  use super::BBST;

  #[test]
  fn test_insert() {
    let mut tree: Treap<u64> = Treap::new();
    tree.insert(10 as u64);
    let x = tree.root.unwrap();
    assert_eq!(x.key, 10 as u64);
  }
}
