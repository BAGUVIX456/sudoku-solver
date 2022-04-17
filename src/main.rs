use anyhow::bail;

mod sudoku;
use sudoku::Sudoku;

fn main() -> Result<(), anyhow::Error> {
    let mut sudoku = Sudoku::new("c:/users/gumbi/desktop/sudoku.xlsx".to_string())?; // remember to remove hardcoded path
    Ok(())
}
