use serde::de::DeserializeOwned;
use serde::Serialize;
// save JSON files in the appdata directory
use serde_json::{from_reader, to_writer_pretty};
use std::fs::File;
use std::io::Result;

/**
 * Note: do not include '.json' when you pass in a value for `name`
 */
pub fn save<T>(name: &str, data: T) -> Result<()>
where
    T: Serialize,
{
    let path = super::helper_methods::get_path(format!("{}.json", name).as_str());
    let file = File::create(path)?;
    to_writer_pretty(file, &data)?;
    Ok(())
}

/**
 * Note: do not include '.json' when you pass in a value for `name`
 */
pub fn load<T>(name: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let path = super::helper_methods::get_path(format!("{}.json", name).as_str());
    let file = File::open(path)?;
    let data: T = from_reader(file)?;
    Ok(data)
}
