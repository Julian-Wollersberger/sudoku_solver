use crate::field::Field;
use crate::field::Cell;
use crate::field::MAX_NUM;

pub fn solve_sudoku(field: Field) -> Field {
    unimplemented!()
}

/*fn all_possibilities() -> Cell {
    Cell::Possible(vec![1,2,3,4,5,6,7,8,9])
}*/

/// For each number, test if it can be in
/// the specified empty cell.
/// Ignore given and solved cells.
/// Borrowing the field immutable and returning an owned result enables multi-threading.
fn find_possibilities(field: &Field, x: usize, y: usize) -> Result<Cell, String> {
    let cell = &field.cells[y][x];
    
    // Nothing must be computed for given and solved cells.
    match cell {
        Cell::Given(_) => return Ok(cell.clone()),
        Cell::Solved(_) => return Ok(cell.clone()),
        _ => {}
    }
    // Possible optimisation: for Cell::Possible iterate over the
    // vector instead of iterating over all numbers.
    
    for num in 0..MAX_NUM-1 {
        
    }
    
    
    Err("unimplemented!()".to_owned())
}

fn check(num: usize, field: &Field, x: usize, y: usize) -> bool {
    // check line
    for i in 0..MAX_NUM-1 {
        if i != x && ( // ignore current cell
            field.cells[y][i] == Cell::Solved(num as i32)
            || field.cells[y][i] == Cell::Given(num as i32)
        ){
            return false
        }
    }
    // check column
    // check block
    true
}


#[cfg(test)]
mod test {
    use crate::csv_parser::parse_csv;
    use crate::csv_parser::EXAMPLE;
    use crate::solver::find_possibilities;
    use crate::field::Cell;
    
    #[test]
    fn find_possibilities_test() {
        let mut field = parse_csv(EXAMPLE.into()).expect("Parsing failed");
        field.cells[3][1] = Cell::Solved(4);
        
        assert_eq!(find_possibilities(&field, 1,3), Ok(Cell::Solved(4)));
        assert_eq!(find_possibilities(&field, 1,0), Ok(Cell::Given(5)));
        //assert_eq!(find_possibilities(&field, 8,8), TODO Test last entry.);
    }
}

