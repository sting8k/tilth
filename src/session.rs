//! Per-invocation expansion tracker.
//!
//! Records which match locations have already been inlined so the same source
//! body is not re-expanded during cascade fallback.

use std::collections::HashSet;
use std::path::Path;
use std::sync::Mutex;

pub struct Session {
    expanded: Mutex<HashSet<String>>, // "path:line" → already inlined
}

impl Session {
    pub fn new() -> Self {
        Session {
            expanded: Mutex::new(HashSet::new()),
        }
    }

    pub fn is_expanded(&self, path: &Path, line: u32) -> bool {
        let key = format!("{}:{}", path.display(), line);
        self.expanded
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .contains(&key)
    }

    pub fn record_expand(&self, path: &Path, line: u32) {
        let key = format!("{}:{}", path.display(), line);
        self.expanded
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(key);
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}
