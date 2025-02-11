fn map<B, F>(self, f: F) -> Map<Self, F>    // 🚧
where
    Self: Sized,
    F: FnMut(Self::Item) -> B,
{
    Map::new(self, f)
}
