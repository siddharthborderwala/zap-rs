use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct Zap {
    store: HashMap<String, String>,
}

impl Zap {
    pub fn new() -> Zap {
        Zap {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.store.get(key).map(|v| v.to_string())
    }

    pub fn has(&self, key: String) -> bool {
        self.store.contains_key(&key)
    }

    pub fn delete(&mut self, key: String) {
        self.store.remove(&key);
    }

    pub fn list(&self) -> impl Iterator<Item = (&String, &String)> {
        self.store.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // test set
    #[test]
    fn test_set_and_get() {
        let mut zap = Zap::new();
        zap.set("name".to_string(), "John".to_string());
        assert_eq!(zap.get("name"), Some("John".to_string()));
    }

    // test has
    #[test]
    fn test_has() {
        let mut zap = Zap::new();
        zap.set("name".to_string(), "John".to_string());
        assert_eq!(zap.has("name".to_string()), true);
    }

    // test delete
    #[test]
    fn test_delete() {
        let mut zap = Zap::new();
        zap.set("name".to_string(), "John".to_string());
        zap.delete("name".to_string());
        assert_eq!(zap.has("name".to_string()), false);
    }
}
