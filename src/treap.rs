use std::cmp::Ordering;
use std::fmt::Debug;
use std::mem::{replace, swap};

use rand::Rng;

use super::BST;
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
  pub fn new(seed: u8) -> Treap<K> {
    Treap {
      root: None,
      rng: rand::SeedableRng::from_seed([seed; 32]),
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
      Some(t) => match item.key.cmp(&t.key) {
        Ordering::Less => {
          Treap::_insert(item, &mut t.lchild);
          if t.lchild.as_deref().unwrap().priority > t.priority {
            Treap::rotate_right(t);
          }
        }
        Ordering::Greater => {
          Treap::_insert(item, &mut t.rchild);
          if t.rchild.as_deref().unwrap().priority > t.priority {
            Treap::rotate_left(t);
          }
        }
        Ordering::Equal => {}
      },
    }
  }
  fn _find(key: &K, tree: &Option<Box<Node<K>>>) -> bool {
    match tree {
      None => false,
      Some(t) => match key.cmp(&t.key) {
        Ordering::Less => Treap::_find(key, &t.lchild),
        Ordering::Greater => Treap::_find(key, &t.rchild),
        Ordering::Equal => true,
      },
    }
  }
  fn root_delete(tree: &mut Option<Box<Node<K>>>) {
    match tree {
      None => {}
      Some(t) => match &t.lchild {
        None => match &t.rchild {
          None => *tree = None,
          Some(_) => {
            Treap::rotate_left(t);
            Treap::root_delete(&mut t.lchild);
          }
        },
        Some(l) => match &t.rchild {
          None => {
            Treap::rotate_right(t);
            Treap::root_delete(&mut t.rchild);
          }
          Some(r) => match l.priority.cmp(&r.priority) {
            Ordering::Equal => {}
            Ordering::Less => {
              Treap::rotate_left(t);
              Treap::root_delete(&mut t.lchild);
            }
            Ordering::Greater => {
              Treap::rotate_right(t);
              Treap::root_delete(&mut t.rchild);
            }
          },
        },
      },
    }
  }
  fn _delete(key: &K, tree: &mut Option<Box<Node<K>>>) {
    match tree {
      None => {}
      Some(t) => match key.cmp(&t.key) {
        Ordering::Less => Treap::_delete(key, &mut t.lchild),
        Ordering::Greater => Treap::_delete(key, &mut t.rchild),
        Ordering::Equal => Treap::root_delete(tree),
      },
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
  #[cfg(test)]
  fn get_seq_in_order(tree: &Box<Node<K>>, seq: &mut Vec<K>)
  where
    K: Copy,
  {
    match &tree.lchild {
      None => {}
      Some(l) => {
        Treap::get_seq_in_order(l, seq);
      }
    }
    seq.push(tree.key);
    match &tree.rchild {
      None => {}
      Some(r) => {
        Treap::get_seq_in_order(r, seq);
      }
    }
  }
  #[cfg(test)]
  pub fn invalid_key(&self) -> bool
  where
    K: Copy,
  {
    let mut seq: Vec<K> = Vec::new();
    Treap::get_seq_in_order(self.root.as_ref().unwrap(), &mut seq);
    let mut ng = false;
    for i in 1..seq.len() - 1 {
      ng |= seq[i] >= seq[i + 1];
    }
    ng
  }
}

impl<K: Ord + Debug> BST<K> for Treap<K> {
  fn insert(&mut self, key: K) {
    let item: Box<Node<K>> = Box::new(Node {
      key,
      priority: self.rng.gen(),
      lchild: None,
      rchild: None,
    });
    Treap::_insert(item, &mut self.root);
  }
  fn find(&self, key: K) -> bool {
    Treap::_find(&key, &self.root)
  }
  fn delete(&mut self, key: K) {
    Treap::_delete(&key, &mut self.root);
  }
}

#[cfg(test)]
mod tests {
  use super::Treap;
  use super::BST;
  use assert_str::assert_str_eq;

  #[test]
  fn test_insert() {
    let mut tree: Treap<u64> = Treap::new(77);
    tree.insert(10);
    assert_str_eq!(tree.print(), "[10()()]");
    assert_eq!(tree.invalid_priority(), false);
    assert_eq!(tree.invalid_key(), false);
  }
  #[test]
  fn test_insert_2() {
    let mut tree: Treap<u64> = Treap::new(77);
    tree.insert(10);
    assert_str_eq!(tree.print(), "[10()()]");
    tree.insert(50);
    assert_str_eq!(tree.print(), "[10()([50()()])]");
    assert_eq!(tree.invalid_priority(), false);
    assert_eq!(tree.invalid_key(), false);
  }
  #[test]
  fn test_insert_3() {
    let mut tree: Treap<u64> = Treap::new(77);
    tree.insert(10);
    assert_str_eq!(tree.print(), "[10()()]");
    tree.insert(50);
    assert_str_eq!(tree.print(), "[10()([50()()])]");
    tree.insert(5);
    assert_str_eq!(tree.print(), "[10([5()()])([50()()])]");
    assert_eq!(tree.invalid_priority(), false);
    assert_eq!(tree.invalid_key(), false);
    tree.insert(5);
    assert_str_eq!(tree.print(), "[10([5()()])([50()()])]");
    assert_eq!(tree.invalid_priority(), false);
    assert_eq!(tree.invalid_key(), false);
    tree.insert(10);
    assert_str_eq!(tree.print(), "[10([5()()])([50()()])]");
    assert_eq!(tree.invalid_priority(), false);
    assert_eq!(tree.invalid_key(), false);
  }

  #[test]
  fn test_rotate() {
    let mut tree: Treap<u64> = Treap::new(77);
    tree.insert(10);
    assert_str_eq!(tree.print(), "[10()()]");
    assert_eq!(tree.invalid_priority(), false);
    assert_eq!(tree.invalid_key(), false);
    tree.insert(50);
    assert_str_eq!(tree.print(), "[10()([50()()])]");
    assert_eq!(tree.invalid_priority(), false);
    assert_eq!(tree.invalid_key(), false);
    tree.insert(5);
    assert_str_eq!(tree.print(), "[10([5()()])([50()()])]");
    assert_eq!(tree.invalid_priority(), false);
    assert_eq!(tree.invalid_key(), false);
    tree.insert(100);
    assert_str_eq!(tree.print(), "[10([5()()])([50()([100()()])])]");
    assert_eq!(tree.invalid_priority(), false);
    assert_eq!(tree.invalid_key(), false);
    tree.insert(200);
    assert_str_eq!(tree.print(), "[10([5()()])([200([50()([100()()])])()])]");
    assert_eq!(tree.invalid_priority(), false);
    assert_eq!(tree.invalid_key(), false);
    tree.insert(400);
    assert_str_eq!(
      tree.print(),
      "[400([10([5()()])([200([50()([100()()])])()])])()]"
    );
    assert_eq!(tree.invalid_priority(), false);
    assert_eq!(tree.invalid_key(), false);
    tree.insert(300);
    assert_str_eq!(
      tree.print(),
      "[400([10([5()()])([200([50()([100()()])])([300()()])])])()]"
    );
    assert_eq!(tree.invalid_priority(), false);
    assert_eq!(tree.invalid_key(), false);
    tree.insert(35);
    assert_str_eq!(
      tree.print(),
      "[400([10([5()()])([35()([200([50()([100()()])])([300()()])])])])()]"
    );
    assert_eq!(tree.invalid_priority(), false);
    assert_eq!(tree.invalid_key(), false);
  }

  #[test]
  fn test_find() {
    let mut tree: Treap<u64> = Treap::new(77);
    assert_eq!(tree.find(0), false);
    tree.insert(10);
    assert_eq!(tree.find(0), false);
    assert_eq!(tree.find(10), true);
    tree.insert(50);
    assert_eq!(tree.find(0), false);
    assert_eq!(tree.find(10), true);
    assert_eq!(tree.find(50), true);
  }
  #[test]
  fn test_delete() {
    let mut tree: Treap<u64> = Treap::new(77);
    assert_eq!(tree.find(0), false);
    tree.insert(10);
    assert_eq!(tree.find(0), false);
    assert_eq!(tree.find(10), true);
    tree.delete(10);
    assert_eq!(tree.find(0), false);
    assert_eq!(tree.find(10), false);
    tree.insert(50);
    assert_eq!(tree.find(0), false);
    assert_eq!(tree.find(10), false);
    assert_eq!(tree.find(50), true);
  }
  #[test]
  fn test_delete_2() {
    let mut tree: Treap<u64> = Treap::new(77);
    tree.insert(10);
    tree.insert(50);
    tree.insert(5);
    tree.insert(100);
    tree.insert(200);
    tree.insert(400);
    tree.insert(300);
    tree.insert(35);
    assert_str_eq!(
      tree.print(),
      "[400([10([5()()])([35()([200([50()([100()()])])([300()()])])])])()]"
    );
    tree.delete(50);
    assert_str_eq!(
      tree.print(),
      "[400([10([5()()])([35()([200([100()()])([300()()])])])])()]"
    );
    assert_eq!(tree.invalid_priority(), false);
    assert_eq!(tree.invalid_key(), false);
    tree.delete(35);
    assert_str_eq!(
      tree.print(),
      "[400([10([5()()])([200([100()()])([300()()])])])()]"
    );
    assert_eq!(tree.invalid_priority(), false);
    assert_eq!(tree.invalid_key(), false);
    tree.delete(400);
    assert_str_eq!(tree.print(), "[10([5()()])([200([100()()])([300()()])])]");
    assert_eq!(tree.invalid_priority(), false);
    assert_eq!(tree.invalid_key(), false);
    tree.delete(200);
    assert_str_eq!(tree.print(), "[10([5()()])([300([100()()])()])]");
    assert_eq!(tree.invalid_priority(), false);
    assert_eq!(tree.invalid_key(), false);
    tree.delete(10);
    assert_str_eq!(tree.print(), "[5()([300([100()()])()])]");
    assert_eq!(tree.invalid_priority(), false);
    assert_eq!(tree.invalid_key(), false);
  }
}
