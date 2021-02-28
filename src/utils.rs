pub trait Sorted {
    fn sorted(&self) -> Self;
}

impl<T> Sorted for Vec<T> where T: Clone + Ord {
    fn sorted(&self) -> Self {
        let mut ret = self.to_vec();
        ret.sort();
        ret
    }
}
