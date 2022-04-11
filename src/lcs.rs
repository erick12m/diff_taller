use std::cmp;

/// Implementa los algoritmos de LCS y print_diff
pub struct LCS;
/// Implementación del algoritmo LCS
/// Devuelve la grilla generada por el algoritmo
/// # Example
/// ```
/// use lcs::LCS;
/// let lines1 = &vec!["igual".to_string(), "dif".to_string()];
/// let lines2 = &vec!["igual".to_string(), "hola mundo".to_string()];
/// let grid = LCS::lcs(&lines1, &lines2);
/// ```
impl LCS {
    pub fn lcs(lines1: &[String], lines2: &[String]) -> Vec<Vec<usize>> {
        let total_rows = lines1.len();
        let total_columns = lines2.len();
        let mut grid = vec![vec![0; total_columns + 1]; total_rows + 1];
        for (row, item1) in lines1.iter().enumerate().take(total_rows) {
            for (col, item2) in lines2.iter().enumerate().take(total_columns) {
                if item1 == item2 {
                    grid[row + 1][col + 1] = grid[row][col] + 1;
                } else {
                    grid[row + 1][col + 1] = cmp::max(grid[row + 1][col], grid[row][col + 1]);
                }
            }
        }
        grid
    }

    /// Implementación del algoritmo print_diff.
    /// Utiliza la grilla computada por el algoritmos LCS
    /// Imprime por stdin el diff de los dos archivos.
    /// # Example
    /// ```
    /// use lcs::LCS;
    /// let lines1 = &vec!["igual".to_string(), "dif".to_string()];
    /// let lines2 = &vec!["igual".to_string(), "hola mundo".to_string()];
    /// let grid = LCS::lcs(&lines1, &lines2);0
    /// LCS::print_diff(&grid, lines1, lines2, lines1.len(), lines2.len());
    /// ```
    pub fn print_diff(
        grid: &Vec<Vec<usize>>,
        lines1: &Vec<String>,
        lines2: &Vec<String>,
        i: usize,
        j: usize,
    ) {
        if i > 0 && j > 0 && lines1[i - 1] == lines2[i - 1] {
            LCS::print_diff(grid, lines1, lines2, i - 1, j - 1);
            println!("   {}", lines1[i - 1]);
        } else if j > 0 && (i == 0 || grid[i][j - 1] >= grid[i - 1][j]) {
            LCS::print_diff(grid, lines1, lines2, i, j - 1);
            println!(">  {}", lines2[j - 1]);
        } else if i > 0 && (j == 0 || grid[i][j - 1] < grid[i - 1][j]) {
            LCS::print_diff(grid, lines1, lines2, i - 1, j);
            println!("<  {}", lines1[i - 1]);
        } else {
            println!();
        }
    }
}

#[test]
fn test_lcr() {
    let lines1 = &vec!["igual".to_string(), "dif".to_string()];
    let lines2 = &vec!["igual".to_string(), "hola mundo".to_string()];
    let grid = LCS::lcs(lines1, lines2);
    LCS::print_diff(&grid, lines1, lines2, lines1.len(), lines2.len());
}
