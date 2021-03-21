use std::fmt::Debug;
use std::mem::{replace, swap};

use rand::Rng;

use super::BBST;

// https://faculty.washington.edu/aragon/pubs/rst96.pdf
// Figure 3
#[derive(Debug)]
struct Node<K: Ord> {
  key: K,
  priority: u64,
  lchild: Option<Box<Node<K>>>,
  rchild: Option<Box<Node<K>>>,
}

pub struct Treap<K: Ord + Debug> {
  root: Option<Box<Node<K>>>,
  rng: rand::rngs::StdRng,
}

impl<K: Ord + Debug> Treap<K> {
  pub fn new() -> Treap<K> {
    Treap {
      root: None,
      rng: rand::SeedableRng::from_seed([77; 32]),
    }
  }

  fn rotate_right(tree: &mut Box<Node<K>>) {
    let lnode = tree.lchild.as_mut().unwrap();
    let llnode = lnode.lchild.take();
    let lrnode = lnode.rchild.take();
    let mut rnode = replace(&mut tree.lchild, llnode);
    swap(
      &mut tree.as_mut().key,
      &mut rnode.as_deref_mut().unwrap().key,
    );
    swap(
      &mut tree.as_mut().priority,
      &mut rnode.as_deref_mut().unwrap().priority,
    );
    rnode.as_deref_mut().unwrap().lchild = lrnode;
    rnode.as_deref_mut().unwrap().rchild = tree.rchild.take();
    tree.rchild = rnode;
  }

  fn rotate_left(tree: &mut Box<Node<K>>) {
    let rnode = tree.rchild.as_mut().unwrap();
    let rlnode = rnode.lchild.take();
    let rrnode = rnode.rchild.take();
    let mut lnode = replace(&mut tree.rchild, rrnode);
    swap(
      &mut tree.as_mut().key,
      &mut lnode.as_deref_mut().unwrap().key,
    );
    swap(
      &mut tree.as_mut().priority,
      &mut lnode.as_deref_mut().unwrap().priority,
    );
    lnode.as_deref_mut().unwrap().lchild = tree.lchild.take();
    lnode.as_deref_mut().unwrap().rchild = rlnode;
    tree.lchild = lnode;
  }

  fn _insert(item: Box<Node<K>>, tree: &mut Option<Box<Node<K>>>) {
    match tree {
      None => {
        *tree = Some(item);
      }
      Some(t) => {
        if item.key < t.key {
          Treap::_insert(item, &mut t.lchild);
          if t.lchild.as_deref().unwrap().priority > t.priority {
            Treap::rotate_right(t);
          }
        } else if item.key > t.key {
          Treap::_insert(item, &mut t.rchild);
          if t.rchild.as_deref().unwrap().priority > t.priority {
            Treap::rotate_left(t);
          }
        }
      }
    }
  }
  #[cfg(test)]
  fn _print(tree: &Box<Node<K>>) -> String {
    let mut message = String::from("[");
    message = [message, format!("{:?}(", tree.key)].concat();
    match &tree.lchild {
      None => {}
      Some(l) => {
        message = [message, Treap::_print(&l)].concat();
      }
    }
    message = [message, String::from(")(")].concat();
    match &tree.rchild {
      None => {}
      Some(r) => {
        message = [message, Treap::_print(&r)].concat();
      }
    }
    [message, String::from(")]")].concat()
  }
  #[cfg(test)]
  pub fn print(&self) -> String {
    Treap::_print(self.root.as_ref().unwrap())
  }
  #[cfg(test)]
  fn _invalid_priority(tree: &Box<Node<K>>) -> bool {
    let mut ng = false;
    match &tree.lchild {
      None => {}
      Some(l) => {
        ng |= l.priority > tree.priority;
        ng |= Treap::_invalid_priority(l);
      }
    }
    match &tree.rchild {
      None => {}
      Some(r) => {
        ng |= r.priority > tree.priority;
        ng |= Treap::_invalid_priority(r);
      }
    }
    ng
  }
  #[cfg(test)]
  pub fn invalid_priority(&self) -> bool {
    Treap::_invalid_priority(self.root.as_ref().unwrap())
  }
}

impl<K: Ord + Debug> BBST<K> for Treap<K> {
  fn insert(&mut self, key: K) {
    let item: Box<Node<K>> = Box::new(Node {
      key,
      priority: self.rng.gen(),
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
  use assert_str::assert_str_eq;

  #[test]
  fn test_insert() {
    let mut tree: Treap<u64> = Treap::new();
    tree.insert(10 as u64);
    assert_str_eq!(tree.print(), "[10()()]");
    assert_eq!(tree.invalid_priority(), false);
  }
  #[test]
  fn test_insert_2() {
    let mut tree: Treap<u64> = Treap::new();
    tree.insert(10 as u64);
    assert_str_eq!(tree.print(), "[10()()]");
    tree.insert(50 as u64);
    assert_str_eq!(tree.print(), "[10()([50()()])]");
    assert_eq!(tree.invalid_priority(), false);
  }
  #[test]
  fn test_insert_3() {
    let mut tree: Treap<u64> = Treap::new();
    tree.insert(10 as u64);
    assert_str_eq!(tree.print(), "[10()()]");
    tree.insert(50 as u64);
    assert_str_eq!(tree.print(), "[10()([50()()])]");
    tree.insert(5 as u64);
    assert_str_eq!(tree.print(), "[10([5()()])([50()()])]");
    assert_eq!(tree.invalid_priority(), false);
  }

  #[test]
  fn test_rotate() {
    let mut tree: Treap<u64> = Treap::new();
    tree.insert(10 as u64);
    assert_str_eq!(tree.print(), "[10()()]");
    assert_eq!(tree.invalid_priority(), false);
    tree.insert(50 as u64);
    assert_str_eq!(tree.print(), "[10()([50()()])]");
    assert_eq!(tree.invalid_priority(), false);
    tree.insert(5 as u64);
    assert_str_eq!(tree.print(), "[10([5()()])([50()()])]");
    assert_eq!(tree.invalid_priority(), false);
    tree.insert(100 as u64);
    assert_str_eq!(tree.print(), "[10([5()()])([50()([100()()])])]");
    assert_eq!(tree.invalid_priority(), false);
    tree.insert(200 as u64);
    assert_str_eq!(tree.print(), "[10([5()()])([200([50()([100()()])])()])]");
    assert_eq!(tree.invalid_priority(), false);
    tree.insert(400 as u64);
    assert_str_eq!(
      tree.print(),
      "[400([10([5()()])([200([50()([100()()])])()])])()]"
    );
    assert_eq!(tree.invalid_priority(), false);
    tree.insert(300 as u64);
    assert_str_eq!(
      tree.print(),
      "[400([10([5()()])([200([50()([100()()])])([300()()])])])()]"
    );
    assert_eq!(tree.invalid_priority(), false);
    tree.insert(35 as u64);
    assert_str_eq!(
      tree.print(),
      "[400([10([5()()])([35()([200([50()([100()()])])([300()()])])])])()]"
    );
    assert_eq!(tree.invalid_priority(), false);
  }
}
