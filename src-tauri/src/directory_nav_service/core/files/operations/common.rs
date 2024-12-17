use std::fs;
use std::path::Path;

// Operations such as moving files, deleting, etc. the basics
// TODO: provide Tauri commands

/// Given that `source_path` points to a directory or file path, this function will move it into the `target_dir`
pub fn move_path_into_directory(target_dir: &Path, source_path: &Path) -> std::io::Result<()> {
    // Ensure the target directory exists
    if !target_dir.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Target directory does not exist",
        ));
    }

    // Extract the file or directory name from the source path
    let file_name = source_path.file_name().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Source path does not have a valid file or directory name",
        )
    })?;

    // Build the destination path
    let destination = target_dir.join(file_name);

    // Move the file or directory to the target location
    fs::rename(source_path, destination)?;

    Ok(())
}

pub fn delete_path(path: &str) -> Result<(), trash::Error> {
    trash::delete(path)
}
