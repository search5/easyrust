fn all<F>(&mut self, f: F) -> bool  // 🚧
where
    F: FnMut(Self::Item) -> bool,
