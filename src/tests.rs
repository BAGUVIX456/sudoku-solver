use crate::Sudoku;
use std::process;

#[test]
fn reading_to_table() -> Result<(), anyhow::Error> {
    let mut sudoku = Sudoku::new();
    sudoku.get_table()?;
    let temp: Vec<u8> = vec![8, 6, 0, 2, 4, 3, 5, 0, 0];

    assert!(sudoku.table.contains(&temp));
    Ok(())
}

#[test]
fn find_empty_cells() {
    let mut sudoku = Sudoku::new();
    sudoku.get_table().unwrap();
    sudoku.get_empty_cells();
    let temp: Vec<u8> = vec![7];

    assert!(sudoku.empty_cells.contains_key(&(1, 1, 0)));
    assert_eq!(sudoku.empty_cells.get(&(1, 1, 0)), Some(&temp));
}

#[test]
fn fill_places() {
    let mut sudoku = Sudoku::new();
    sudoku.get_table();
    sudoku.get_empty_cells();
    sudoku.fill_sure_places();

    assert_eq!(8, sudoku.table[2][2])
}

#[test]
fn get_possible_values() {
    let mut sudoku = Sudoku::new();
    sudoku.get_table().unwrap();
    assert_eq!(sudoku.possible_vals(&(0, 8, 2)), vec![7, 9]);
}
