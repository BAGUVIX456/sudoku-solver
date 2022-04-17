use anyhow::{bail, Context};
use calamine::{open_workbook, Reader, Xlsx};
use std::collections::HashSet;
use itertools::izip;
use array_tool::vec::Intersect;

pub struct Sudoku {
    table: Vec<Vec<u8>>,
    empty_cells: Vec<[usize; 3]>,
    // Possible row vals
    row_vals: Vec<Vec<u8>>,
    // Possible column vals
    col_vals: Vec<Vec<u8>>,
    // Possible values in each region
    region_vals: Vec<Vec<u8>>,
}

impl Sudoku {
    pub fn new(path: String) -> Result<Sudoku, anyhow::Error> {
        let mut sudoku = Sudoku {
            table: vec![],
            empty_cells: vec![],
            row_vals: vec![],
            col_vals: vec![],
            region_vals: vec![],
        };

        let mut workbook: Xlsx<_> = open_workbook(path).context("Failed to get xlsx file")?;
        let sheet = workbook.worksheet_range("Sheet1").unwrap()?;

        for row in sheet.rows() {
            let mut temp = Vec::new();
            for value in row {
                match value.get_float() {
                    Some(val) => temp.push(val as u8),
                    None => bail!("Improper input in xlsx file"),
                }
            }
            sudoku.table.push(temp);
        }

        if sudoku.table.is_empty() {
            bail!("Enter contents in Sheet 1 of xlsx file");
        }

        println!("Finished reading xlsx file contents");
        Ok(sudoku)
    }

    pub fn is_valid(&mut self) -> Result<(), anyhow::Error> {
        let valid = |slice: &Vec<u8>| {
            let mut unique = HashSet::new();

            for element in slice {
                if *element != 0  && !unique.insert(element) {
                    bail!("Entered sudoku puzzle is invalid!");
                }
            }
            Ok(())
        };

        let mut transposed_table = [[0u8; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                transposed_table[i][j] = self.table[j][i];
            }
        }

        let mut region_vals: Vec<Vec<u8>> = vec![];
        for chunk in self.table.chunks(3) {
            for i in 0..9 {
                let mut temp = Vec::new();
                chunk.iter()
                    .for_each(|row| temp.push(row[3*i..3*i+3].to_vec()));
                region_vals.push(temp.into_iter().flatten().collect());
            }
        }

        for (row, col, region) in izip!(&self.table, &transposed_table, &region_vals) {
            valid(row)?;
            valid(&col.to_vec())?;
            valid(region)?;
        }
        Ok(())
    }

    pub fn get_empty_cells(&mut self) {
        for position in 0..9 {
            self.table[position].iter().enumerate()
                .filter(|(.., &index)| index == 0)
                .map(|(col,..)| [position, col, position/3*3+col/3 ])
                .for_each(|coords| self.empty_cells.push(coords));
        }
        println!("Finished finding empty positions");
    }

    pub fn fill_sure_places(&mut self) -> Result<(), anyhow::Error> {
        for &[row, col, region] in &self.empty_cells {
            let possible_vals = self.row_vals[row]
                .intersect(self.col_vals[col].intersect(self.region_vals[region]));
            if possible_vals.len() == 1 {
                self.table[row][col] = possible_vals[0];
                let row_2 = &mut self.row_vals[row];
                let col_2 = &mut self.col_vals[col];
                let region_2 = &mut self.region_vals[region];
                // dude, using row_2 most probably won't work. try testing. also, the fuck is happening in line 100?
                row_2.swap_remove(row_2.iter().position(|&x| x == possible_vals[0]).unwrap());
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
