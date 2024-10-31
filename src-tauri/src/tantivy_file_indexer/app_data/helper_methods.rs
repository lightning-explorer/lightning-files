use std::path::{Path, PathBuf};
use std::fs;
use dirs::data_dir;

/**
 * returns a path to the file you specify, except the file is now inside the AppData directory. No files are created
 */
pub fn get_path(path:&str)->PathBuf{
    let app_data = data_dir().expect("Could not find AppData directory");
    let app_path = app_data.join("DesktopSearch");
    let new_path = app_path.join(path);

    // Ensure that the App's AppData directory is there
    if !app_path.exists() {
        fs::create_dir_all("DesktopSearch").expect("could not create DesktopSearch directory");
    }
    return new_path;
}

/**
 * Doesn't do anything if the path already exists. Creates the file you specify in AppData
 */
pub fn create_path(path:&str)->PathBuf{
    let app_data = data_dir().expect("Could not find AppData directory");
    let app_path = app_data.join("DesktopSearch");
    let new_path = app_path.join(path);

    // Ensure that the App's AppData directory is there
    if !app_path.exists() {
        fs::create_dir_all("DesktopSearch").expect("could not create DesktopSearch directory");
    }

    if !new_path.exists(){
        if let Some(parent) = Path::new(&new_path).parent() {
            fs::create_dir_all(parent).expect("failed to creat directories");
        }

        // Create the empty database file
        fs::File::create(&new_path).expect("failed to create path");
    }
    return new_path;
}