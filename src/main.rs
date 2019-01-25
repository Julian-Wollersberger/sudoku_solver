use crate::csv_parser::parse_csv;
use crate::csv_parser::EXAMPLE;
use crate::solver::solve_sudoku;

mod csv_parser;
mod field;
mod solver;

// Für jede Zelle eine Liste der noch möglichen Zahlen, die passen könnten.
// Wenn nur eine übrig bleibt, dann ist diese fix.

fn main() {
    let sudoku = parse_csv(EXAMPLE.into()).expect("Parsing failed");
    println!("{}", solve_sudoku(sudoku).unwrap());
}


// Not all sudokus can be solved with elimination.
// Others need trial and error. Rekursive or try  flat?
// If no number fits, -> impossible




