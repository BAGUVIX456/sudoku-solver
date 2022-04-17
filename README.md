# <p align="center"> Sudoku Solver </p>

A CLI app made using Rust to solve simple sudoku puzzles. Currently, this application can solve sudokus with only 1 solution. Its working has been tested only on Windows, and it would be of great help if you test the application in your OS as well!

To use this software:
```
1. Download sudoku_solver.exe into your computer.
2. Create a new excel spreadsheet. Enter the values of the sudoku puzzle with 0 in all the empty places.
3. Open a command terminal in your system and run the executable.
``` 
<u>Note:</u> Double-clicking on the executable won't work! Run the executable from the terminal itself.

<img src="https://user-images.githubusercontent.com/85876638/161526789-365c2d90-1435-478e-b1aa-c72dd5cff462.png" width="78%">
<img src="https://user-images.githubusercontent.com/85876638/161528291-0e3eece3-88cc-42c6-b36e-6513987fae14.jpg" width="75%">

### Changes to be made:
```
1. Ability to parse command line arguments using clap.
2. Implementing a backtracking algorithm to be able to solve any type of sudoku puzzle, not only the ones with 1 solution.
3. Using ndarray (about which I came to know only after finishing this entire freaking thing) to simplify code.
4. Adding more features other than just solving the puzzle.
```
