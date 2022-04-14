use crate::region;
use anyhow::{bail, Context};
use array_tool::vec::Intersect;
use calamine::{open_workbook, Reader, Xlsx};
use std::collections::{HashMap, HashSet};

pub struct Sudoku {
    /// The actual sudoku table.
    pub table: Vec<Vec<u8>>,

    /// A hashmap with the key being the coordinates of the empty position and and value being the possible values.
    /// The first two elements of the key tuple gives the row and column index of the empty position in regular
    /// zero based indexing. The third element of the key tuple gives the region (the smaller squares in the
    /// sudoku table). The region indexing is zero based and is from the top-left square to the bottom-right
    pub empty_cells: HashMap<(usize, usize, usize), Vec<u8>>,
}

impl Sudoku {
    /// Creates a new Sudoku instance.
    ///
    /// `
    /// let puzzle = Sudoku::new();
    /// `
    pub fn new() -> Sudoku {
        Sudoku {
            table: vec![],
            empty_cells: HashMap::new(),
        }
    }

    /// Reads the puzzle from the given XLSX file
    pub fn get_table(&mut self, path: String) -> Result<(), anyhow::Error> {
        let mut workbook: Xlsx<_> = open_workbook(path).context("Failed to get xlsx file")?;
        let mut return_val = Vec::new();

        let result = workbook.worksheet_range("Sheet1").unwrap()?;

        for row in result.rows() {
            let mut temp = Vec::new();

            for value in row {
                match value.get_float() {
                    Some(val) => temp.push(val as u8),
                    None => bail!("Improper input in XLSX file"),
                }
            }
            return_val.push(temp);
        }
        if return_val.is_empty() {
            bail!("Please enter contents in Sheet 1 of XLSX file");
        }

        self.table = return_val;
        println!("Finished reading XLSX file contents");
        Ok(())
    }

    // figure out how to integrate this with the find values function
    pub fn is_valid(&mut self) -> Result<(), anyhow::Error> {
        let valid = |slice: Vec<u8>| {
            let mut unique = HashSet::new();
            if slice.iter().cloned().all( |x| unique.insert(x)) {
                bail!("Entered sudoku puzzle is invalid!");
            }
            Ok(())
        };

        let mut transposed_table = [[0u8; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                transposed_table[i][j] = self.table[j][i];
            }
        };

        Ok(()) 
    }

    /// Traverses the table to find the empty cells and the possible values of each empty cell
    pub fn get_empty_cells(&mut self) {
        // This block of code is run only once, when the a shiny new instance of Sudoku is passed to get_empty_cells()
        if self.empty_cells.is_empty() {
            let mut empty_positions = Vec::new();

            // This loop finds all the coordinates of the empty positions and collects it in empty_positions
            for position in 0..9 {
                self.table[position]
                    .iter()
                    .enumerate()
                    .filter(|x| *x.1 == 0)
                    .map(|coords| (position, coords.0, region(position, coords.0)))
                    .for_each(|coords| empty_positions.push(coords));
            }

            // Fills values into the empty_cells hashmap
            for coordinates in empty_positions {
                self.empty_cells
                    .insert(coordinates, self.possible_vals(&coordinates));
            }
            println!("Finished finding empty positions");
        }
        // This block goes through the empty_cells hashmap instead of the entire table.
        else {
            let key_list: Vec<(usize, usize, usize)> = self.empty_cells.keys().copied().collect();
            for key in key_list {
                self.empty_cells.insert(key, self.possible_vals(&key));
            }
        }
    }

    /// Takes in the coordinates of an empty position, and then finds the possible values in the row,
    /// column, and the region of the empty cell separately. The common values in the possible row, column and
    /// region values is returned from this function.
    pub fn possible_vals(&self, (row, column, region): &(usize, usize, usize)) -> Vec<u8> {
        let mut transposed_table = [[0u8; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                transposed_table[i][j] = self.table[j][i];
            }
        }

        let required_values = |slice: &[Vec<u8>], arg| {
            let mut possible_values = Vec::new();
            for row in slice {
                row.iter()
                    .enumerate()
                    .filter(|x| x.0 >= 3 * arg && x.0 <= 3 * arg + 2)
                    .for_each(|x| possible_values.push(*x.1));
            }
            possible_values
        };

        let pos_vals;
        if vec![0, 1, 2].contains(region) {
            pos_vals = required_values(&self.table[0..3], region);
        } else if vec![3, 4, 5].contains(region) {
            pos_vals = required_values(&self.table[3..6], &(region - 3));
        } else {
            pos_vals = required_values(&self.table[6..8], &(region - 6));
        }

        let check_for_possible_vals = |table_element: &Vec<u8>| {
            (1..10)
                .into_iter()
                .filter(|x| !table_element.contains(x))
                .collect()
        };

        let row_vals: Vec<u8> = check_for_possible_vals(&self.table[*row]);
        let col_vals = check_for_possible_vals(&transposed_table[*column].to_vec());
        let region_vals = check_for_possible_vals(&pos_vals);

        row_vals.intersect(col_vals.intersect(region_vals))
    }

    /// Fills all the empty cells having only 1 possible value that can be inserted into it and
    /// returns the keys of all the filled positions
    pub fn fill_sure_places(&mut self) -> Result<Vec<(usize, usize, usize)>, anyhow::Error> {
        let mut filled_positions = Vec::new();

        for (key, values) in &self.empty_cells {
            if values.len() == 1 {
                self.table[key.0][key.1] = values[0];
                filled_positions.push(*key);
            }
        }
        if filled_positions.is_empty() {
            bail!("This sudoku puzzle has more than one solution and cannot be solved")
        }
        Ok(filled_positions)
    }

    pub fn print_table(&self) {
        println!();
        for row in &self.table {
            println!("{:?}", row);
        }
    }
}
