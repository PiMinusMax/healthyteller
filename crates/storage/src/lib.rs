//! Local storage for profiles, settings and cached source data. Nothing leaves the device
//! (AGENTS.md rule 4): the browser keeps it in `localStorage`, native apps in a folder.
//!
//! Values are stored as JSON per key. Encryption of the profile and the SQLite database
//! for source data follow in later milestones (see docs/adr/0002-app-aufbau.md).

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

use serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageError(pub String);

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "storage error: {}", self.0)
    }
}

impl std::error::Error for StorageError {}

pub trait KeyValue {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&self, key: &str, value: &str) -> Result<(), StorageError>;
    fn remove(&self, key: &str) -> Result<(), StorageError>;
}

/// Typed access on top of a key-value backend.
pub struct Store {
    backend: Box<dyn KeyValue>,
}

impl Store {
    pub fn new(backend: impl KeyValue + 'static) -> Self {
        Self {
            backend: Box::new(backend),
        }
    }

    /// The default store of the platform, or memory if none is available.
    pub fn platform(app_dir: Option<std::path::PathBuf>) -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = app_dir;
            match BrowserStore::new() {
                Some(store) => Self::new(store),
                None => Self::new(MemoryStore::default()),
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            match app_dir.and_then(|dir| FileStore::new(dir).ok()) {
                Some(store) => Self::new(store),
                None => Self::new(MemoryStore::default()),
            }
        }
    }

    /// Reads a value; unreadable or outdated JSON counts as missing.
    pub fn load<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        serde_json::from_str(&self.backend.get(key)?).ok()
    }

    pub fn save<T: Serialize>(&self, key: &str, value: &T) -> Result<(), StorageError> {
        let json = serde_json::to_string(value).map_err(|e| StorageError(e.to_string()))?;
        self.backend.set(key, &json)
    }

    pub fn remove(&self, key: &str) -> Result<(), StorageError> {
        self.backend.remove(key)
    }
}

#[derive(Default)]
pub struct MemoryStore {
    values: RefCell<HashMap<String, String>>,
}

impl KeyValue for MemoryStore {
    fn get(&self, key: &str) -> Option<String> {
        self.values.borrow().get(key).cloned()
    }

    fn set(&self, key: &str, value: &str) -> Result<(), StorageError> {
        self.values
            .borrow_mut()
            .insert(key.to_owned(), value.to_owned());
        Ok(())
    }

    fn remove(&self, key: &str) -> Result<(), StorageError> {
        self.values.borrow_mut().remove(key);
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
pub struct BrowserStore {
    storage: web_sys::Storage,
}

#[cfg(target_arch = "wasm32")]
impl BrowserStore {
    const PREFIX: &'static str = "healthyteller.";

    pub fn new() -> Option<Self> {
        let storage = web_sys::window()?.local_storage().ok()??;
        Some(Self { storage })
    }
}

#[cfg(target_arch = "wasm32")]
impl KeyValue for BrowserStore {
    fn get(&self, key: &str) -> Option<String> {
        self.storage
            .get_item(&format!("{}{key}", Self::PREFIX))
            .ok()?
    }

    fn set(&self, key: &str, value: &str) -> Result<(), StorageError> {
        self.storage
            .set_item(&format!("{}{key}", Self::PREFIX), value)
            .map_err(|e| StorageError(format!("{e:?}")))
    }

    fn remove(&self, key: &str) -> Result<(), StorageError> {
        self.storage
            .remove_item(&format!("{}{key}", Self::PREFIX))
            .map_err(|e| StorageError(format!("{e:?}")))
    }
}

/// One JSON file per key in the app's data folder.
#[cfg(not(target_arch = "wasm32"))]
pub struct FileStore {
    dir: std::path::PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
impl FileStore {
    pub fn new(dir: std::path::PathBuf) -> std::io::Result<Self> {
        std::fs::create_dir_all(&dir)?;
        Ok(Self { dir })
    }

    fn path(&self, key: &str) -> std::path::PathBuf {
        let safe: String = key
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        self.dir.join(format!("{safe}.json"))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl KeyValue for FileStore {
    fn get(&self, key: &str) -> Option<String> {
        std::fs::read_to_string(self.path(key)).ok()
    }

    fn set(&self, key: &str, value: &str) -> Result<(), StorageError> {
        // Write to a temporary file first so a crash never leaves half a file.
        let path = self.path(key);
        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, value).map_err(|e| StorageError(e.to_string()))?;
        std::fs::rename(&tmp, &path).map_err(|e| StorageError(e.to_string()))
    }

    fn remove(&self, key: &str) -> Result<(), StorageError> {
        match std::fs::remove_file(self.path(key)) {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(StorageError(e.to_string())),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Serialize, serde::Deserialize)]
    struct Settings {
        accepted: bool,
        name: String,
    }

    fn round_trip(store: &Store) {
        let value = Settings {
            accepted: true,
            name: "Test".into(),
        };
        assert_eq!(store.load::<Settings>("settings"), None);
        store.save("settings", &value).unwrap();
        assert_eq!(store.load::<Settings>("settings"), Some(value));
        store.remove("settings").unwrap();
        assert_eq!(store.load::<Settings>("settings"), None);
        store.remove("settings").unwrap();
    }

    #[test]
    fn memory_store_round_trip() {
        round_trip(&Store::new(MemoryStore::default()));
    }

    #[test]
    fn file_store_round_trip() {
        let dir = std::env::temp_dir().join(format!("healthyteller-test-{}", std::process::id()));
        round_trip(&Store::new(FileStore::new(dir.clone()).unwrap()));
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn unreadable_json_counts_as_missing() {
        let store = Store::new(MemoryStore::default());
        store.backend.set("settings", "{not json").unwrap();
        assert_eq!(store.load::<Settings>("settings"), None);
    }

    #[test]
    fn file_names_are_sanitised() {
        let store = FileStore::new(std::env::temp_dir()).unwrap();
        assert!(store.path("../etc/passwd").ends_with("___etc_passwd.json"));
    }
}
