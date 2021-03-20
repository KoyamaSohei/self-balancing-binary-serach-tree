pub mod treap;

pub trait BBST<K> {
    fn insert(&mut self, key: K);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
