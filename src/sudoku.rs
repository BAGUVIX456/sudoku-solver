use anyhow::{bail, Context};
use array_tool::vec::Intersect;
use calamine::{open_workbook, Reader, Xlsx};
use itertools::izip;
use std::collections::HashSet;

pub struct Sudoku {
    /// Holds the actual sudoku table
    pub table: Vec<Vec<u8>>,

    /// Contains the coordinates of all the empty positions in the table.
    /// The three elements of each [usize; 3] are the row index, column index, and
    /// the region index, respectively. The 9 regions of the sudoku table are
    /// taken in regular zero based indexing, starting from the top-left and ending at
    /// the bottom-right.
    pub empty_cells: Vec<[usize; 3]>,

    /// Contains the possible values that can be inserted into each row
    pub row_vals: Vec<Vec<u8>>,
    /// Possible values that can be inserted into each column
    pub col_vals: Vec<Vec<u8>>,
    /// Possible values that can be inserted into each region
    pub region_vals: Vec<Vec<u8>>,
}

impl Sudoku {

    /// Creates a new instance of Sudoku and reads the xlsx file into self.table
    pub fn from(path: String) -> Result<Sudoku, anyhow::Error> {
        let mut sudoku = Sudoku {
            table: vec![],
            empty_cells: vec![],
            row_vals: vec![],
            col_vals: vec![],
            region_vals: vec![],
        };

        let mut workbook: Xlsx<_> = open_workbook(path).context("Failed to get xlsx file")?;
        let sheet = workbook.worksheet_range("Sheet1").unwrap()?;

        // to read the excel file row by row and push into
        // sudoku.table after each row is read.
        for row in sheet.rows() {
            let mut temp = Vec::new();
            for value in row {
                match value.get_float() {
                    Some(val) => temp.push(val as u8),
                    None => bail!("Improper input in xlsx file"),
                }
            }

            if temp.len() != 9 {
                bail!("Sudoku table has less columns than expected");
            }
            sudoku.table.push(temp);
        }

        if sudoku.table.is_empty() {
            bail!("Enter contents in Sheet 1 of xlsx file");
        } else if sudoku.table.len() != 9 {
            bail!("Sudoku puzzle has less rows than expected!");
        }

        println!("Finished reading xlsx file contents");
        Ok(sudoku)
    }

    /// Checks the validity of the entered sudoku table, and also fills values into sudoku.row_vals, sudoku.col_vals
    /// and sudoku.region_vals
    pub fn is_valid(&mut self) -> Result<(), anyhow::Error> {
 
        let possible_vals = |slice: &Vec<u8>| {
            let mut unique = HashSet::new();
            for element in slice {
                if *element != 0 && !unique.insert(element) {
                    bail!("Entered sudoku puzzle is invalid!");
                }
            }
            Ok((1..10).into_iter().filter(|x| !slice.contains(x)).collect())
        };

        let mut transposed_table = [[0u8; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                transposed_table[i][j] = self.table[j][i];
            }
        }

        let mut region_vals: Vec<Vec<u8>> = vec![];
        for chunk in self.table.chunks(3) {
            for i in 0..3 {
                let mut temp = Vec::new();
                chunk.iter()
                    .for_each(|row| temp.push(row[3 * i .. 3 * i + 3].to_vec()));
                region_vals.push(temp.into_iter().flatten().collect());
            }
        }

        for (row, col, region) in izip!(&self.table, &transposed_table, &region_vals) {
            self.row_vals.push(possible_vals(row)?);
            self.col_vals.push(possible_vals(&col.to_vec())?);
            self.region_vals.push(possible_vals(region)?);
        }
        Ok(())
    }

    pub fn get_empty_cells(&mut self) {
        for position in 0..9 {
            self.table[position].iter().enumerate()
                .filter(|(.., &index)| index == 0)
                .map(|(col, ..)| [position, col, position / 3 * 3 + col / 3])
                .for_each(|coords| self.empty_cells.push(coords));
        }
        println!("Finished finding empty positions");
    }

    pub fn fill_sure_places(&mut self) -> Result<(), anyhow::Error> {
        let mut removed_values = Vec::new();

        for &[row, col, region] in &self.empty_cells {
            let a = &mut self.row_vals[row];
            let b = &mut self.col_vals[col];
            let c = &mut self.region_vals[region];
            let possible_vals = a.intersect(b.intersect(c.to_vec()));

            if possible_vals.len() == 1 {
                self.table[row][col] = possible_vals[0];
                removed_values.push([row, col, region]);

                let get_position =
                    |slice: &Vec<u8>| match slice.iter().position(|&x| x == possible_vals[0]) {
                        Some(val) => Ok(val),
                        None => bail!("Given sudoku puzzle is invalid"),
                    };
                a.swap_remove(get_position(a)?);
                b.swap_remove(get_position(b)?);
                c.swap_remove(get_position(c)?);
            }
        }

        if removed_values.is_empty() {
            bail!("I am sorry, this sudoku puzzle cannot be solved by me (yet)!!");
        }

        for &coords in &removed_values {
            match self.empty_cells.iter().position(|&x| x == coords) {
                Some(index) => {
                    self.empty_cells.swap_remove(index);
                }
                None => bail!("Given sudoku puzzle is invalid"),
            }
        }
        Ok(())
    }

    pub fn print_table(&self) {
        println!();
        for row in &self.table {
            println!("{:?}", row);
        }
    }
}
