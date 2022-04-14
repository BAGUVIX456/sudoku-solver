//! A simple CLI application to solve sudoku puzzles.
use anyhow::bail;
use clap::{ArgEnum, Parser};
use human_panic::setup_panic;

/// Some sloppily written tests
#[cfg(test)]
mod tests;

mod sudoku;
use sudoku::Sudoku;

/// This function determines the region of an empty position and is run only once during the execution
/// of the program (in the very first iteration of the loop in main).
fn region(row: usize, col: usize) -> usize {
    let return_val = |row_index| {
        if vec![0, 1, 2].contains(&col) {
            row_index
        } else if vec![3, 4, 5].contains(&col) {
            row_index + 1
        } else {
            row_index + 2
        }
    };

    if vec![0, 1, 2].contains(&row) {
        return_val(0usize)
    } else if vec![3, 4, 5].contains(&row) {
        return_val(3usize)
    } else {
        return_val(6usize)
    }
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    #[clap(
        arg_enum,
        required = true,
        help = "Specifies the operation to be made on the sudoku table\n"
    )]
    command: Commands,

    #[clap(
        required = true,
        help = "Location of the xlsx file containing the sudoku table"
    )]
    file_path: String,
}

#[derive(Clone, ArgEnum)]
enum Commands {
    Solve,
    Check,
}

fn main() -> Result<(), anyhow::Error> {
    setup_panic!();
    let args = Arguments::parse();
    let mut sudoku = Sudoku::new();
    sudoku.get_table(args.file_path)?;

    match args.command {
        Commands::Solve => {
            let mut just_for_counting = 1;

            loop {
                sudoku.get_empty_cells();
                for filled_positions in sudoku.fill_sure_places()? {
                    sudoku.empty_cells.remove(&filled_positions);
                }

                println!(
                    "Finished filling sure positions -> Round {}",
                    just_for_counting
                );
                just_for_counting += 1;

                if sudoku.empty_cells.is_empty() {
                    println!("\nSudoku puzzle solved successfully");
                    sudoku.print_table();
                    println!();
                    break;
                }
            }
            Ok(())
        }
        Commands::Check => bail!("Haven't programmed this part yet!"),
    }
}
