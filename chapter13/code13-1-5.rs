fn height(mut self, height: u32) -> Self {  // 🚧
    self.height = height;
    self
}

fn weight(mut self, weight: u32) -> Self {
    self.weight = weight;
    self
}

fn name(mut self, name: &str) -> Self {
    self.name = name.to_string();
    self
}
