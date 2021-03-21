pub mod treap;

pub trait BST<K> {
    fn insert(&mut self, key: K);
}
