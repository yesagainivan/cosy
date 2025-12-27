use crate::error::CosynError;
use crate::value::Value;
use std::path::Path;

/// Load and merge multiple configuration files.
///
/// The files are loaded in order. Subsequent files override previous ones.
/// Deep merging is performed using `deep_merge`.
///
/// # Example
///
/// ```no_run
/// use cosy::load_and_merge;
/// use std::path::Path;
///
/// let paths = [
///     Path::new("base.cosy"),
///     Path::new("local.cosy"),
/// ];
/// let config = load_and_merge(&paths).unwrap();
/// ```
pub fn load_and_merge(paths: &[&Path]) -> Result<Value, CosynError> {
    let mut merged = Value::Object(indexmap::IndexMap::new());

    for path in paths {
        let content = std::fs::read_to_string(path).map_err(|e| CosynError::Io(e.to_string()))?;

        let mut current = crate::syntax::parser::from_str(&content)?;

        // Resolve includes for this file *before* merging it into the main config.
        let base_dir = path.parent().unwrap_or(Path::new("."));

        crate::include::resolve(&mut current, base_dir)
            .map_err(|e| CosynError::Include(e.to_string()))?;

        crate::merge::merge(&mut merged, current);
    }

    Ok(merged)
}
