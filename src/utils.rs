pub trait Sorted {
    fn sorted(&self) -> Self;
    fn unique(&self) -> Self;
}

impl<T> Sorted for Vec<T> where T: Clone + Ord {
    fn sorted(&self) -> Self {
        let mut ret = self.to_vec();
        ret.sort();
        ret
    }

    fn unique(&self) -> Self {
        let ret = self.to_vec();
        let mut uni = vec![];
        for e in ret.into_iter() { if !uni.contains(&e) { uni.push(e) } }
        uni
    }
}
