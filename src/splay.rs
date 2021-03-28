pub trait SplayTree<T> {
    type Error;

    fn insert(&mut self, value: T);
    fn remove(&mut self, index: usize) -> Result<(), Self::Error>;
    fn size(&self) -> usize;
    fn height(&self) -> usize;
}

