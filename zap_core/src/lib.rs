use hashbrown::HashMap;

pub struct Zap<'a> {
    store: HashMap<&'a str, &'a str>,
}

impl<'a> Zap<'a> {
    pub fn new() -> Zap<'a> {
        Zap {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &'a str, value: &'a str) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.store.get(key).map(|&v| v.to_string())
    }

    pub fn has(&self, key: &'a str) -> bool {
        self.store.contains_key(key)
    }

    pub fn delete(&mut self, key: &'a str) {
        self.store.remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // test set
    #[test]
    fn test_set_and_get() {
        let mut zap = Zap::new();
        zap.set("name", "John");
        assert_eq!(zap.get("name"), Some("John".to_string()));
    }

    // test has
    #[test]
    fn test_has() {
        let mut zap = Zap::new();
        zap.set("name", "John");
        assert_eq!(zap.has("name"), true);
    }

    // test delete
    #[test]
    fn test_delete() {
        let mut zap = Zap::new();
        zap.set("name", "John");
        zap.delete("name");
        assert_eq!(zap.has("name"), false);
    }
}
