fn build(mut self) -> Result<Character, String> { // 🚧
    if self.height < 200 && self.weight < 300 &&
        !self.name.to_lowercase().contains("smurf") {
        self.can_use = true;
        Ok(self)
    } else {
        Err("Could not create character. Characters must have:
1) Height below 200
2) Weight below 300
3) A name that is not Smurf (that is a bad word)".to_string())
    }
}
