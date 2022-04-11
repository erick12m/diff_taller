use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

/// Lee las líneas del archivo especificado en el path y las devuelve en un vector de String.
/// En que cada posición es una línea leída del archivo
/// # Example
/// ```
/// use read::lines_from_file;
/// let lines = read::lines_from_file("/etc/hosts");
/// ```
pub fn lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>, &'static str> {
    let file = File::open(filename).map_err(|_| "File path not found")?;
    let buf = BufReader::new(file);
    Ok(buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect())
}
