pub mod lcs;
pub mod read;
use std::error::Error;

use lcs::LCS;

/// Path de los archivos a leer para hacer el diff
/// Pasados como argumentos por línea de comando
#[derive(Debug)]
pub struct Paths {
    pub file1: String,
    pub file2: String,
}

impl Paths {
    /// Se obtienen los paths pasados por argumentos de la línea de comando
    /// Se chequea inicialment de que se pasen los dos paths como argumentos.
    /// En caso contrario se genera un error.
    /// Si se pasan más de dos paths se tomaran únicamente los dos primeros.
    pub fn new(args: &[String]) -> Result<Paths, Box<dyn Error>> {
        if args.len() < 3 {
            return Err("Missing file(s)".into());
        }
        if args.len() > 3 {
            return Err("Expecting only two files".into());
        }
        Ok(Paths {
            file1: args[1].to_string(),
            file2: args[2].to_string(),
        })
    }
}

/// Con los dos paths de los archivos enviados como argumentos.
/// Se leen ambos archivos y cada uno se guarda en un vector de String
/// Ambos vectores son utilizados para generar la grilla de LCS.
/// Dicha grilla es utilizado para imprimir el diff de los archivos.

pub fn run(config: Paths) -> Result<(), Box<dyn Error>> {
    let lines1 = read::lines_from_file(config.file1)?;
    let lines2 = read::lines_from_file(config.file2)?;
    let grid = LCS::lcs(&lines1, &lines2);
    LCS::print_diff(&grid, &lines1, &lines2, lines1.len(), lines2.len());
    Ok(())
}
