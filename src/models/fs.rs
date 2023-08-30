use serde::Serialize;
use std::error::Error;
use std::path::Path;

pub fn store<T: Serialize>(
    el: &T,
    root: &Path,
    name: &str,
    extension: &str,
) -> Result<(), Box<dyn Error>> {
    let mut path = root.to_path_buf();

    path.push(name);
    path.set_extension(extension);

    log::trace!("Storing {} at {}", name, path.display());

    let serialized = serde_json::to_string(el)?;
    let res = std::fs::write(path, serialized);

    if let Err(e) = res {
        log::error!("Failed to write file: {}", e);
        return Err(Box::new(e));
    }

    Ok(())
}
