use clap::{ArgEnum, Parser};
use human_panic::setup_panic;

mod sudoku;
use sudoku::Sudoku;

#[cfg(test)]
mod tests;

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
        help = "File path of the xlsx file containing the sudoku table"
    )]
    file_path: String,
}

#[derive(ArgEnum, Clone)]
enum Commands {
    Solve,
    Check,
}

fn main() -> Result<(), anyhow::Error> {
    setup_panic!();
    let args = Arguments::parse();
    let mut sudoku = Sudoku::from(args.file_path)?;

    match args.command {
        Commands::Solve => {
            sudoku.is_valid()?;
            sudoku.get_empty_cells();
            let mut just_for_counting = 1;

            while !sudoku.empty_cells.is_empty() {
                sudoku.fill_sure_places()?;
                println!(
                    "Finished filling sure places -> Round {}",
                    just_for_counting
                );
                just_for_counting += 1;
            }

            println!("\nSudoku puzzle solved successfully: ");
            sudoku.print_table();
        }

        Commands::Check => {
            sudoku.is_valid()?;
            println!("\nYour sudoku puzzle is perfectly valid 👍")
        }
    }
    Ok(())
}
