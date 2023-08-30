use serde::Serialize;
use std::error::Error;
use std::path::PathBuf;

pub fn store<T: Serialize>(
    el: &T,
    root: &PathBuf,
    name: &str,
    extension: &str,
) -> Result<(), Box<dyn Error>> {
    let mut path = root.clone();

    path.push(name);
    path.set_extension(extension);

    log::trace!("Storing {} at {}", name, path.display());

    let serialized = serde_json::to_string(el)?;
    let res = std::fs::write(path, serialized);

    match res {
        Err(e) => {
            log::error!("Failed to write file: {}", e);
            return Err(Box::new(e));
        }
        _ => {}
    }

    Ok(())
}

// pub fn iterate_dir(path: &PathBuf) {
//     let dir = std::fs::read_dir(&path);

//     dir.

//   }
