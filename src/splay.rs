use std::boxed::Box;
use std::fmt::{Debug, Display};

pub trait SplayTree<T> {
    type Error;

    fn insert(&mut self, value: T);
    fn remove(&mut self, index: usize) -> Result<(), Self::Error>;
    fn size(&self) -> usize;
    fn height(&self) -> usize;
}

#[derive(Clone, Debug)]
pub struct BoxedSplayTree<T>
where
    T: Sized + Clone + Display + Debug + Eq + Ord,
{
    value: T,
    left: Option<Box<BoxedSplayTree<T>>>,
    right: Option<Box<BoxedSplayTree<T>>>,
}
