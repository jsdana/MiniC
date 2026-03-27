use std::collections::{HashMap, HashSet};

/// Unified parametric environment: maps names to values of type `V`.
/// Both variable bindings and function bindings are stored in the same map.
pub struct Environment<V> {
    bindings: HashMap<String, V>,
}

impl<V: Clone> Environment<V> {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// Bind `name` to `value`, overwriting any existing binding.
    pub fn declare(&mut self, name: impl Into<String>, value: V) {
        self.bindings.insert(name.into(), value);
    }

    /// Look up a binding by name.
    pub fn get(&self, name: &str) -> Option<&V> {
        self.bindings.get(name)
    }

    /// Update an existing binding. Returns `false` if the name is not bound.
    pub fn set(&mut self, name: &str, value: V) -> bool {
        if self.bindings.contains_key(name) {
            self.bindings.insert(name.to_string(), value);
            true
        } else {
            false
        }
    }

    /// Capture a full clone of the current bindings (for function call scoping).
    pub fn snapshot(&self) -> HashMap<String, V> {
        self.bindings.clone()
    }

    /// Replace all bindings with the given snapshot (for function call scoping).
    pub fn restore(&mut self, snapshot: HashMap<String, V>) {
        self.bindings = snapshot;
    }

    /// Return the set of currently bound names (for block-entry capture).
    pub fn names(&self) -> HashSet<String> {
        self.bindings.keys().cloned().collect()
    }

    /// Remove any binding whose name is not in `outer` (for block-exit cleanup).
    pub fn remove_new(&mut self, outer: &HashSet<String>) {
        self.bindings.retain(|k, _| outer.contains(k));
    }
}

impl<V: Clone> Default for Environment<V> {
    fn default() -> Self {
        Self::new()
    }
}
