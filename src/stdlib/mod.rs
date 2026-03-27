use std::collections::HashMap;

use crate::ir::ast::Type;
use crate::interpreter::value::NativeFn;

pub mod io;
pub mod math;

/// A registry entry: MiniC type signature + Rust implementation.
pub struct NativeEntry {
    /// MiniC parameter types (used for arity and type checking).
    pub params: Vec<Type>,
    /// MiniC return type.
    pub return_type: Type,
    /// Rust implementation.
    pub func: NativeFn,
}

/// Maps function names to their native entries.
pub struct NativeRegistry {
    entries: HashMap<String, NativeEntry>,
}

impl NativeRegistry {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, entry: NativeEntry) {
        self.entries.insert(name.to_string(), entry);
    }

    pub fn lookup(&self, name: &str) -> Option<&NativeEntry> {
        self.entries.get(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &NativeEntry)> {
        self.entries.iter()
    }
}

impl Default for NativeRegistry {
    fn default() -> Self {
        let mut r = Self::new();

        // IO
        r.register("print", NativeEntry {
            params: vec![Type::Any],
            return_type: Type::Unit,
            func: io::print_fn,
        });
        r.register("readInt", NativeEntry {
            params: vec![],
            return_type: Type::Int,
            func: io::read_int_fn,
        });
        r.register("readFloat", NativeEntry {
            params: vec![],
            return_type: Type::Float,
            func: io::read_float_fn,
        });
        r.register("readString", NativeEntry {
            params: vec![],
            return_type: Type::Str,
            func: io::read_string_fn,
        });

        // Math
        r.register("pow", NativeEntry {
            params: vec![Type::Float, Type::Float],
            return_type: Type::Float,
            func: math::pow_fn,
        });
        r.register("sqrt", NativeEntry {
            params: vec![Type::Float],
            return_type: Type::Float,
            func: math::sqrt_fn,
        });

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_registry_contains_all_stdlib() {
        let r = NativeRegistry::default();
        assert!(r.lookup("print").is_some());
        assert!(r.lookup("readInt").is_some());
        assert!(r.lookup("readFloat").is_some());
        assert!(r.lookup("readString").is_some());
        assert!(r.lookup("pow").is_some());
        assert!(r.lookup("sqrt").is_some());
    }

    #[test]
    fn test_lookup_unregistered_returns_none() {
        let r = NativeRegistry::default();
        assert!(r.lookup("unknown").is_none());
    }

    #[test]
    fn test_sqrt_entry_signature() {
        let r = NativeRegistry::default();
        let entry = r.lookup("sqrt").unwrap();
        assert_eq!(entry.params, vec![Type::Float]);
        assert_eq!(entry.return_type, Type::Float);
    }

    #[test]
    fn test_print_uses_type_any() {
        let r = NativeRegistry::default();
        let entry = r.lookup("print").unwrap();
        assert_eq!(entry.params, vec![Type::Any]);
    }
}
