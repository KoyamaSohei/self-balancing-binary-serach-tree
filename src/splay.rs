use std::cmp::Ordering;
use std::fmt::Debug;
use std::mem::{replace, swap};

use super::BST;
#[derive(Debug)]
struct Node<K: Ord> {
  key: K,
  lchild: Option<Box<Node<K>>>,
  rchild: Option<Box<Node<K>>>,
}

pub struct Splay<K: Ord + Debug> {
  root: Option<Box<Node<K>>>,
}

impl<K: Ord + Debug> Splay<K> {
  pub fn new() -> Splay<K> {
    Splay { root: None }
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
          Splay::_insert(item, &mut t.lchild);
        }
        Ordering::Greater => {
          Splay::_insert(item, &mut t.rchild);
        }
        Ordering::Equal => {}
      },
    }
  }

  fn splay(key: &K, tree: &mut Option<Box<Node<K>>>) -> bool {
    match tree {
      None => false,
      Some(t) => match key.cmp(&t.key) {
        Ordering::Less => match &mut t.lchild {
          None => false,
          Some(l) => match key.cmp(&l.key) {
            Ordering::Less => match &mut l.lchild {
              None => false,
              Some(ll) => match key.cmp(&ll.key) {
                Ordering::Less => {
                  if !Splay::splay(key, &mut t.lchild) {
                    return false;
                  }
                  Splay::splay(key, tree)
                }
                Ordering::Greater => {
                  if !Splay::splay(key, &mut t.lchild) {
                    return false;
                  }
                  Splay::splay(key, tree)
                }
                Ordering::Equal => {
                  // zig-zig
                  //      t
                  //     /
                  //    l
                  //   /
                  //  ll(target)
                  Splay::rotate_right(t);
                  Splay::rotate_right(t);
                  true
                }
              },
            },
            Ordering::Greater => match &mut l.rchild {
              None => false,
              Some(lr) => match key.cmp(&lr.key) {
                Ordering::Less => {
                  if !Splay::splay(key, &mut t.lchild) {
                    return false;
                  }
                  Splay::splay(key, tree)
                }
                Ordering::Greater => {
                  if !Splay::splay(key, &mut t.lchild) {
                    return false;
                  }
                  Splay::splay(key, tree)
                }
                Ordering::Equal => {
                  // zig-zag
                  //      t
                  //     /
                  //    l
                  //     \
                  //      lr(target)
                  Splay::rotate_left(l);
                  Splay::rotate_right(t);
                  true
                }
              },
            },
            Ordering::Equal => true,
          },
        },
        Ordering::Greater => match &mut t.rchild {
          None => false,
          Some(r) => match key.cmp(&r.key) {
            Ordering::Less => match &mut r.lchild {
              None => false,
              Some(rl) => match key.cmp(&rl.key) {
                Ordering::Less => {
                  if !Splay::splay(key, &mut t.rchild) {
                    return false;
                  }
                  Splay::splay(key, tree)
                }
                Ordering::Greater => {
                  if !Splay::splay(key, &mut t.rchild) {
                    return false;
                  }
                  Splay::splay(key, tree)
                }
                Ordering::Equal => {
                  // zig-zag
                  //      t
                  //       \
                  //        r
                  //       /
                  //      rl(target)
                  Splay::rotate_right(r);
                  Splay::rotate_left(t);
                  true
                }
              },
            },
            Ordering::Greater => match &mut r.rchild {
              None => false,
              Some(rr) => match key.cmp(&rr.key) {
                Ordering::Less => {
                  if !Splay::splay(key, &mut t.rchild) {
                    return false;
                  }
                  Splay::splay(key, tree)
                }
                Ordering::Greater => {
                  if !Splay::splay(key, &mut t.rchild) {
                    return false;
                  }
                  Splay::splay(key, tree)
                }
                Ordering::Equal => {
                  // zig-zig
                  //      t
                  //       \
                  //        r
                  //         \
                  //          rr(target)
                  Splay::rotate_left(t);
                  Splay::rotate_left(t);
                  true
                }
              },
            },
            Ordering::Equal => true,
          },
        },
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
            Splay::rotate_left(t);
            Splay::root_delete(&mut t.lchild);
          }
        },
        Some(_) => match &t.rchild {
          None => {
            Splay::rotate_right(t);
            Splay::root_delete(&mut t.rchild);
          }
          Some(_) => {
            Splay::rotate_left(t);
            Splay::root_delete(&mut t.lchild);
          }
        },
      },
    }
  }
  fn _delete(key: &K, tree: &mut Option<Box<Node<K>>>) {
    match tree {
      None => {}
      Some(t) => match key.cmp(&t.key) {
        Ordering::Less => Splay::_delete(key, &mut t.lchild),
        Ordering::Greater => Splay::_delete(key, &mut t.rchild),
        Ordering::Equal => Splay::root_delete(tree),
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
        message = [message, Splay::_print(&l)].concat();
      }
    }
    message = [message, String::from(")(")].concat();
    match &tree.rchild {
      None => {}
      Some(r) => {
        message = [message, Splay::_print(&r)].concat();
      }
    }
    [message, String::from(")]")].concat()
  }
  #[cfg(test)]
  pub fn print(&self) -> String {
    Splay::_print(self.root.as_ref().unwrap())
  }
  #[cfg(test)]
  fn get_seq_in_order(tree: &Box<Node<K>>, seq: &mut Vec<K>)
  where
    K: Copy,
  {
    match &tree.lchild {
      None => {}
      Some(l) => {
        Splay::get_seq_in_order(l, seq);
      }
    }
    seq.push(tree.key);
    match &tree.rchild {
      None => {}
      Some(r) => {
        Splay::get_seq_in_order(r, seq);
      }
    }
  }
  #[cfg(test)]
  pub fn invalid_key(&self) -> bool
  where
    K: Copy,
  {
    let mut seq: Vec<K> = Vec::new();
    Splay::get_seq_in_order(self.root.as_ref().unwrap(), &mut seq);
    let mut ng = false;
    for i in 1..seq.len() - 1 {
      ng |= seq[i] >= seq[i + 1];
    }
    ng
  }
}

impl<K: Ord + Debug> BST<K> for Splay<K> {
  fn insert(&mut self, key: K) {
    let item: Box<Node<K>> = Box::new(Node {
      key,
      lchild: None,
      rchild: None,
    });
    Splay::_insert(item, &mut self.root);
  }
  fn find(&mut self, key: K) -> bool {
    if !Splay::splay(&key, &mut self.root) {
      return false;
    }
    match key.cmp(&self.root.as_ref().unwrap().key) {
      Ordering::Equal => {}
      Ordering::Less => {
        Splay::rotate_right(&mut self.root.as_mut().unwrap());
      }
      Ordering::Greater => {
        Splay::rotate_left(&mut self.root.as_mut().unwrap());
      }
    }
    true
  }
  fn delete(&mut self, key: K) {
    Splay::_delete(&key, &mut self.root);
  }
}

#[cfg(test)]
mod tests {
  use super::Splay;
  use super::BST;
  use assert_str::assert_str_eq;

  #[test]
  fn test_insert() {
    let mut tree: Splay<u64> = Splay::new();
    tree.insert(10);
    assert_str_eq!(tree.print(), "[10()()]");
    assert_eq!(tree.invalid_key(), false);
  }
  #[test]
  fn test_insert_2() {
    let mut tree: Splay<u64> = Splay::new();
    tree.insert(10);
    assert_str_eq!(tree.print(), "[10()()]");
    tree.insert(50);
    assert_str_eq!(tree.print(), "[10()([50()()])]");
    assert_eq!(tree.invalid_key(), false);
    tree.insert(5);
    assert_str_eq!(tree.print(), "[10([5()()])([50()()])]");
    assert_eq!(tree.invalid_key(), false);
  }
  #[test]
  fn test_find() {
    let mut tree: Splay<u64> = Splay::new();
    tree.insert(10);
    tree.insert(50);
    assert_eq!(tree.find(10), true);
    assert_str_eq!(tree.print(), "[10()([50()()])]");
    assert_eq!(tree.find(50), true);
    assert_str_eq!(tree.print(), "[50([10()()])()]");
    tree.insert(5);
    assert_str_eq!(tree.print(), "[50([10([5()()])()])()]");
    assert_eq!(tree.find(5), true);
    assert_str_eq!(tree.print(), "[5()([10()([50()()])])]");
    assert_eq!(tree.find(100), false);
    assert_str_eq!(tree.print(), "[5()([10()([50()()])])]");
  }
  #[test]
  fn test_find_2() {
    let mut tree: Splay<u64> = Splay::new();
    tree.insert(10);
    tree.insert(50);
    tree.insert(5);
    tree.insert(200);
    tree.insert(400);
    tree.insert(100);
    assert_str_eq!(
      tree.print(),
      "[10([5()()])([50()([200([100()()])([400()()])])])]"
    );
    assert_eq!(tree.invalid_key(), false);
    assert_eq!(tree.find(200), true);
    assert_str_eq!(
      tree.print(),
      "[200([50([10([5()()])()])([100()()])])([400()()])]"
    );
    assert_eq!(tree.invalid_key(), false);
  }
  #[test]
  fn test_delete() {
    let mut tree: Splay<u64> = Splay::new();
    tree.insert(10);
    tree.insert(50);
    tree.insert(5);
    tree.insert(200);
    tree.insert(400);
    tree.insert(100);
    tree.delete(200);
    assert_str_eq!(tree.print(), "[10([5()()])([50()([400([100()()])()])])]");
    assert_eq!(tree.find(200), false);
    assert_eq!(tree.invalid_key(), false);
    tree.delete(10);
    assert_str_eq!(tree.print(), "[50([5()()])([400([100()()])()])]");
    assert_eq!(tree.find(10), false);
    assert_eq!(tree.invalid_key(), false);
  }
}
