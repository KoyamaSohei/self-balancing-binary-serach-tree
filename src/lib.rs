pub mod splay;
pub mod treap;

pub trait BST<K> {
    fn insert(&mut self, key: K);
    fn find(&mut self, key: K) -> bool;
    fn delete(&mut self, key: K);
}
