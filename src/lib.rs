pub mod treap;

pub trait BST<K> {
    fn insert(&mut self, key: K);
    fn find(&self, key: K) -> bool;
}
