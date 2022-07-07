pub mod tables;

use std::io;
use home;

use crate::constants;

pub fn is_db_exists() -> io::Result<bool> {
    let (_, db_path) = get_db_path()?;

    Ok(std::path::Path::new(db_path.as_str()).exists())
}

pub fn create_db_file() -> io::Result<()> {
    let (db_dir_path, db_path) = get_db_path()?;

    if std::path::Path::new(db_dir_path.as_str()).exists() {
        return Ok(());
    }

    std::fs::create_dir_all(db_dir_path)?;
    std::fs::File::create(db_path)?;

    Ok(())
}

fn get_db_path() -> io::Result<(String, String)> {
    let home_dir = match home::home_dir() {
        Some(dir) => dir,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Not able to get home directory"
            ))
        },
    };

    let dir_path = format!("{}/.{}", home_dir.display(), constants::APP_NAME);
    Ok((String::from(&dir_path), format!("{}/{}", dir_path, constants::DB_FILE_NAME)))
}