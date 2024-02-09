/// Helps keep two correlated vectors in sync.
pub(crate) struct Covec<T,S>(pub(crate) Vec<T>, pub(crate) Vec<S>);

impl<T, S> Covec<T,S> {
    pub(crate) fn push(&mut self, x: T, y: S) {
       self.0.push(x);
       self.1.push(y);
    }

    pub(crate) fn drain1_sync(&mut self) {
        let len0 = self.0.len();
        let len1 = self.1.len();
        let _ = self.1.drain(0..len1.saturating_sub(len0));
    }
}

impl<T,S> Default for Covec<T,S> {
    fn default() -> Self {
        Self(vec![], vec![])
    }
}
