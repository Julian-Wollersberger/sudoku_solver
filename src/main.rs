mod csv_parser;

// Für jede Zelle eine Liste der noch möglichen Zahlen, die passen könnten.
// Wenn nur eine übrig bleibt, dann ist diese fix.
const MAX_NUM: usize = 9;

#[derive(Debug, Eq, PartialEq)]
enum Cell {
    Given(i32),
    //Solved(i32),
    Empty,
    //Possible(Vec<i32>),
}

fn main() {
    parse_csv(EXAMPLE.into()).expect("Parsing failed");
}


// Not all sudokus can be solved with elimination.
// Others need trial and error. Rekursive or try  flat?
// If no number fits, -> impossible




