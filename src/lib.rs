pub mod treap;

pub trait BST<K> {
    fn insert(&mut self, key: K);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
