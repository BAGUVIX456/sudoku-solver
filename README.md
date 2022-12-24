# <p align="center"> Sudoku Solver </p>

A CLI app made using Rust to solve simple sudoku puzzles. Currently, this application can solve sudokus having only 1 solution. Its working has been tested only on Windows, and it would be of great help if you test the application in your OS as well!

To use this software:
   1. Download sudoku.exe from the Releases tab.
   2. Create a new Excel spreadsheet file and enter the sudoku table in Sheet 1, with 0 entered in all the empty places. Note the location of this file.
  
       <img src="https://user-images.githubusercontent.com/85876638/161526789-365c2d90-1435-478e-b1aa-c72dd5cff462.png" width="80%">
   3. Open your favourite terminal and `cd` yourself into the directory in which sudoku.exe is installed.
  
       ``` 
       cd <insert file path here> 
       ```
       The usage of this command might be slightly different in different operating systems.
   4. Run `sudoku solve <file_path>` to solve the sudoku puzzle or `sudoku check <file_path>` to check its validity!
   5. Use `sudoku --help` for help on how to use the commands.
   
   <img src="https://user-images.githubusercontent.com/85876638/163722234-75d554f4-82a0-4c72-a29e-65caf0bc7c11.png" width="80%">

<b> Note: Double-clicking on the executable won't work! Run the executable from the terminal itself. </b>

### Planned features for future versions:
  1. Implementing a better algorithm to be able to solve any kind of sudoku puzzle.
  2. Making error messages more helpful.
  3. Specifying reason for failure in check command
  3. Beautifying solution output.
